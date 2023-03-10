FROM debian:buster
LABEL maintainer GiGa infosystems <info@giga-infosystems.com>

ENV JOBS=32
USER root
RUN apt-get update && apt-get install -y wget gnupg && apt-get upgrade -y
ENV APT_KEY_DONT_WARN_ON_DANGEROUS_USAGE=1
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential autoconf libtool automake\
    ca-certificates \
    libssl-dev \
    git \
    curl \
    sudo \
    gcc g++ \
    pkg-config \
    libaio1 libaio-dev \
    unzip openssh-client

USER root

# Use runners uid/gid for the user we install rust for.
# Defaults match the first created user for most linux distros.
# The files generated by this build will get the uid/gid of these args.
ARG USER_ID=1000
ARG GROUP_ID=1000

# Create a user and group with the runner's choice of uid/gid and map it to rust/rust.
#
# Note: `no-log-init` seems to be a workaround for docker hanging with large uid/gid
#       and was found in multiple related stackoverflow links.
#
#       User needs NOPASSWD sudo to start postgres.
RUN groupadd --gid ${GROUP_ID} rust && \
    useradd --no-log-init --create-home -G sudo --uid ${USER_ID} --gid rust rust && \
    echo "rust ALL=(root) NOPASSWD:ALL" > /etc/sudoers.d/rust && \
    chmod 0440 /etc/sudoers.d/rust

# We want to install rustup and set up env as rust.
USER rust

# Install rust toolchain of the runners choice.
ARG RUST_VERSION
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/home/rust/.cargo/bin:$PATH
RUN rustup toolchain install $RUST_VERSION && \
    rustup component add rustfmt && \
    rustup component add clippy

# Install cargo deny to check licenses
RUN cargo install cargo-deny
ENV LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

RUN mkdir /home/rust/src
WORKDIR /home/rust/src

VOLUME ["/home/rust/src"]
CMD ["/bin/bash"]