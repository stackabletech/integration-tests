import requests
import sys
import logging

if __name__ == "__main__":
    result = 0

    log_level = 'DEBUG' ### if args.debug else 'INFO'
    logging.basicConfig(level=log_level, format='%(asctime)s %(levelname)s: %(message)s', stream=sys.stdout)

    response = requests.get("http://derby-druid-coordinator-default:8081/druid/indexer/v1/tasks")
    if response.status_code != 200:
        result = 1
    print(f'tasks {response}')

    headers = {'Accept': 'application/json', 'Content-Type': 'application/json'}
    response = requests.post("derby-druid-coordinator-default:8081/druid/indexer/v1/task",
                              data=open('druid-quickstartimport.json', 'rb'),
                              headers=headers)
    print(f'ingest {response}')
    if response.status_code != 200:
        result = 1

    sys.exit(result)
