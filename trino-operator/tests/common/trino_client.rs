//! A Trino client for testing purposes
//!
//! Alternatives:
//! - [prusto](https://crates.io/crates/prusto)
//!   A feature-rich Trino client library which requires Rust Nightly.
//! - [trino](https://crates.io/crates/trino)
//!   A Trino client library with nearly the same limited feature set as this one but it does not
//!   provide a blocking API and does not support authentication headers.
use anyhow::Result;
use integration_test_commons::stackable_operator::k8s_openapi::serde_json;
use reqwest::{
    blocking::{Body, Client, Request},
    Certificate,
};
use serde::de::DeserializeOwned;

/// A Trino client for testing purposes
pub struct TrinoClient {
    address: String,
    username: String,
    password: String,
    http_client: Client,
}

impl TrinoClient {
    /// Creates a new Trino client.
    pub fn new(address: &str, username: &str, password: &str, pem_certificate: &[u8]) -> Self {
        TrinoClient {
            address: address.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
            http_client: Client::builder()
                .add_root_certificate(Certificate::from_pem(pem_certificate).unwrap())
                .build()
                .unwrap(),
        }
    }

    /// Executes the given Trino statement.
    pub fn execute<T>(&self, statement: &str) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let request = self
            .http_client
            .post(format!("https://{}/v1/statement", self.address))
            .basic_auth(&self.username, Some(&self.password))
            .header("X-Trino-User", &self.username)
            .body(statement.to_owned())
            .build()?;
        let mut response = self.call(request)?;

        let mut data = Vec::new();

        while let Some(serde_json::Value::String(next_uri)) = response.get("nextUri") {
            let request = self.http_client.get(next_uri).build()?;
            response = self.call(request)?;

            if let Some(serde_json::Value::Array(rows)) = response.get("data") {
                let mut data_part = rows
                    .iter()
                    .map(|entry| serde_json::from_value::<T>(entry.to_owned()).unwrap())
                    .collect();
                data.append(&mut data_part);
            }
        }

        Ok(data)
    }

    /// Executes the given HTTP request and returns the response body parsed as JSON value.
    ///
    /// The request and response are printed to the standard output. The value of the authorization
    /// header is shown as "Sensitive".
    fn call(&self, request: Request) -> Result<serde_json::Value> {
        println!(
            "HTTP Request\n\
            ------------\n\
            {verb} {url} {version}\n\
            {headers}\
            {body}",
            verb = request.method(),
            url = request.url(),
            version = format!("{:?}", request.version()),
            headers = request
                .headers()
                .iter()
                .map(|(key, value)| format!("{}: {:?}\n", key, value))
                .collect::<String>(),
            body = std::str::from_utf8(request.body().and_then(Body::as_bytes).unwrap_or_default())
                .map(|body| if body.is_empty() {
                    String::new()
                } else {
                    format!("\n{}\n", body)
                })?,
        );

        let response = self.http_client.execute(request)?;

        let response_version = response.version();
        let response_status = response.status();
        let response_headers = response.headers().clone();
        let response_body = response.text()?;
        println!(
            "HTTP Response\n\
            -------------\n\
            {version} {status}\n\
            {headers}\
            \n{body}",
            version = format!("{:?}", response_version),
            status = response_status,
            headers = response_headers
                .iter()
                .map(|(key, value)| format!("{}: {:?}\n", key, value))
                .collect::<String>(),
            body = response_body,
        );

        let response_json = serde_json::from_str(&response_body)?;
        Ok(response_json)
    }
}
