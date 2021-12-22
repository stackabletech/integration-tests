use anyhow::{anyhow, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::{Duration, Instant};

/// If pods are set to "ready" the cluster may still need time to balance to be fully ready.
/// Therefore we resend the 4lw (if not successful) in the defined timeout period.
const FOUR_LETTER_WORD_REQUEST_TIMEOUT: u64 = 10;
/// Tests if server is running in a non-error state. The server will respond with imok if it is running.
/// Otherwise it will not respond at all.
pub const ARE_YOU_OK: &str = "ruok";
/// Positive response for the "ruok" command.
pub const I_AM_OK: &str = "imok";

/// Send "ruok" to a pod and check if the response is "imok"
pub fn send_4lw_i_am_ok(version: &str, host: &str) -> Result<()> {
    // 3.4.14 answers with "imok", while 3.5.X onwards the command is mirrored
    // which results in the "ruok" response we have to differentiate here.
    let ver = if Version::parse(version)? > Version::parse("3.5.2")? {
        ARE_YOU_OK.to_string()
    } else {
        I_AM_OK.to_string()
    };

    // The cluster requires some time to balance after all pods are "ready". This may result in four
    // letter word requests to fail shortly after. He we resend the four letter word every two
    // seconds for the defined timeout FOUR_LETTER_WORD_REQUEST_TIMEOUT.
    let now = Instant::now();
    let mut last_response = Ok("<no-response-received>".to_string());

    while now.elapsed().as_secs() < Duration::from_secs(FOUR_LETTER_WORD_REQUEST_TIMEOUT).as_secs()
    {
        last_response = send_4lw(version, ARE_YOU_OK, host);

        if last_response.is_err() || last_response.as_ref().unwrap() != &ver {
            println!(
                "[{}] Received: {:?}. Will resend command.",
                ARE_YOU_OK, last_response
            );
            thread::sleep(Duration::from_secs(2));
            continue;
        }

        return Ok(());
    }

    Err(anyhow!(
        "Could not verify cluster status response via [{}] within the specified timeout [{}]: {:?}",
        ARE_YOU_OK,
        FOUR_LETTER_WORD_REQUEST_TIMEOUT,
        last_response
    ))
}

/// This sends the "four letter word" in order to check if the cluster is ready or to get
/// statistics. We have to differentiate between the ZooKeeper versions.
/// Up to 3.5.2 the standard four letter word can be used.
/// From 3.5.3 onwards we query the admin server via http request.
pub fn send_4lw(version: &str, four_letter_word: &str, host: &str) -> Result<String> {
    if Version::parse(version)? > Version::parse("3.5.2")? {
        send_cmd_to_admin_server(four_letter_word, host)
    } else {
        send_4lw_to_host(four_letter_word, host)
    }
}

/// Create a TCP connection to the given host name (format: <host>:<port>) and send the
/// provided 4 letter command (e.g. "ruok") and return the received response.
/// The response is hardcoded to be 4 letters as well, even though some of the four
/// letter commands of ZooKeeper return more data (e.g. stat).
/// This only works until version 3.5.2. With version 3.5.3 this functionality was moved to the
/// admin server. To keep up the four letter words you have to whitelist the required commands
/// in the zoo.cfg via: "4lw.commands.whitelist=*" ("*" for all commands to be whitelisted)
fn send_4lw_to_host(four_letter_word: &str, host: &str) -> Result<String> {
    let mut stream = TcpStream::connect(host)?;

    println!("Writing [{}] to [{}]", four_letter_word, host);
    stream.write_all(four_letter_word.as_bytes())?;
    stream.flush()?;

    let mut response = [0u8; 4];
    stream.read_exact(&mut response)?;

    let received = std::str::from_utf8(&response).expect("valid utf8");

    println!("Received: {}", received);
    Ok(received.to_string())
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct AdminServerResponse {
    pub command: String,
    pub error: Option<String>,
}

/// Send a http request to "http://HOST:PORT/commands/COMMAND"
/// It will return a JSON response containing at least:
/// {
///     command: "some_string",
///     error: "some_error"
/// }
/// If no errors occur, "null" (which in serde parses to None) is returned
fn send_cmd_to_admin_server(command: &str, host: &str) -> Result<String> {
    // TODO: Support https
    let url = format!("http://{}/commands/{}", host, command);

    println!("Requesting [{}]", url);
    let mut res = reqwest::blocking::get(&url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let response: AdminServerResponse = serde_json::from_str(&body)?;

    println!("Received: {}", body);

    if response.error.is_none() {
        return Ok(response.command);
    }

    Err(anyhow!(
        "Received error while executing command to admin server: {:?}",
        response.error
    ))
}
