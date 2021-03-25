# Since the "offical" Rust image doesn't have OpenCV 4, let's do it manually
# Based on Ubuntu 20
# See https://github.com/liuchong/docker-rustup/blob/master/dockerfiles/stable/Dockerfile

# NOTE - run the Dockerfile from the parent dir using:
# run_docker.sh

FROM ubuntu:20.04

# Get packages
# A little hack for tzdata: https://stackoverflow.com/questions/44331836/apt-get-install-tzdata-noninteractive
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    clang \
    libclang-dev \
    libopencv-contrib4.2 \
    libopencv-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Update PATH for cargo
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Install Rust toolchain, see https://rust-lang.github.io/rustup/installation/other.html
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

# Copy over files (.git is needed otherwise build.rs fails)
WORKDIR /usr/src/bitmapflow/
COPY rust .
COPY .git .git

# Compile bitmapflow
RUN cargo build --release
