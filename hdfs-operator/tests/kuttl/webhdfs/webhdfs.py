import requests
import sys
import logging

if __name__ == "__main__":
    result = 0
    command = sys.argv[1]

    log_level = 'DEBUG' ### if args.debug else 'INFO'
    logging.basicConfig(level=log_level, format='%(asctime)s %(levelname)s: %(message)s', stream=sys.stdout)

    if command == "ls":
        http_code = requests.get("http://hdfs-namenode-default-0:9870/webhdfs/v1/testdata.txt?user.name=stackable&op=LISTSTATUS").status_code
        result = http_code == 200
    elif command == "create":
        files = {'file': ('testdata.txt', open('/tmp/testdata.txt', 'rb'), 'text/plain', {'Expires': '0'})}
        http_code = requests.put("http://hdfs-namenode-default-0:9870/webhdfs/v1/testdata.txt?user.name=stackable&op=CREATE", files=files, allow_redirects=True).status_code
        result = http_code == 201
    else:
        result = 1

    sys.exit(result)
