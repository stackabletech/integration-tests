FROM debian:10

# install helpful tools
RUN apt-get update
RUN apt-get install curl python3 unzip wget vim git gcc libssl-dev pkg-config -y

# install Helm and the Stackable Helm repos
RUN curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash -s -
RUN helm repo add stackable-dev https://repo.stackable.tech/repository/helm-dev/
RUN helm repo add stackable-stable https://repo.stackable.tech/repository/helm-stable/
RUN helm repo update

# install kubectl
RUN wget -O /tmp/kubectl "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
RUN install -o root -g root -m 0755 /tmp/kubectl /usr/local/bin/kubectl
RUN rm /tmp/kubectl

# install Rust
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH "/root/.cargo/bin:$PATH"

# copy test sources
RUN mkdir /integration-tests
COPY . /integration-tests


# build tests
WORKDIR /integration-tests
RUN cargo build --tests

# make script executable
RUN chmod u+x /integration-tests/run-test-in-docker.sh

# start Docker test script
ENTRYPOINT ["/integration-tests/run-test-in-docker.sh"]
