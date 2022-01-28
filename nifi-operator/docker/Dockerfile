FROM python:3.7

# Install prerequisites
RUN pip install requests

# Install tests
RUN mkdir /tests
COPY tests/kuttl/smoke/*.py /tests/

