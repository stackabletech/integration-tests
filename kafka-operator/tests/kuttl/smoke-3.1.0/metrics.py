import requests
import sys
import logging

if __name__ == "__main__":
    result = 0

    log_level = 'DEBUG'  # if args.debug else 'INFO'
    logging.basicConfig(level=log_level, format='%(asctime)s %(levelname)s: %(message)s', stream=sys.stdout)

    http_code = requests.get("http://simple-kafka-broker-default:9606").status_code
    if http_code != 200:
        result = 1

    sys.exit(result)