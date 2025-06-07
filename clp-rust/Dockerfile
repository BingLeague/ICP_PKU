FROM ubuntu:22.04

ENV NVM_DIR=/root/.nvm
ENV NVM_VERSION=v0.39.1
ENV NODE_VERSION=20.11.0
ENV DFXVM_INIT_YES=yes
ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
ENV RUST_VERSION=1.78.0
ENV DFX_VERSION=0.20.1
# ENV http_proxy=http://192.168.3.17:9999
# ENV https_proxy=http://192.168.3.17:9999
# ENV all_proxy=http://192.168.3.17:10000
# RUN sed -i 's/security\.ubuntu\.com/mirrors.aliyun.com/g; s/archive\.ubuntu\.com/mirrors.aliyun.com/g' /etc/apt/sources.list

# Install a basic environment needed for our build tools
RUN apt -yq update && \
    apt -yq install --no-install-recommends curl ca-certificates \
    build-essential pkg-config libssl-dev llvm-dev liblmdb-dev clang cmake rsync libunwind8

# Install Node.js using nvm
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin:${PATH}"
RUN curl --fail -sSf https://raw.githubusercontent.com/creationix/nvm/${NVM_VERSION}/install.sh | bash
RUN . "${NVM_DIR}/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "${NVM_DIR}/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "${NVM_DIR}/nvm.sh" && nvm alias default v${NODE_VERSION}
# Install dfx
RUN sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
# Install Rust and Cargo
ENV PATH=/opt/cargo/bin:${PATH}
RUN curl --fail https://sh.rustup.rs -sSf \
        | sh -s -- -y --default-toolchain ${RUST_VERSION}-x86_64-unknown-linux-gnu --no-modify-path
    
RUN rustup default ${RUST_VERSION}-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown
RUN cargo install ic-wasm
RUN cargo install candid-extractor
RUN cargo install cargo-audit

COPY . /canister
WORKDIR /canister
RUN cargo build --target wasm32-unknown-unknown