FROM python:3

# install required Python modules
COPY python_modules /
RUN pip install --no-cache-dir -r python_modules

# copy main Python script
COPY preprocessor.py /

CMD [ "python", "/preprocessor.py" ]
