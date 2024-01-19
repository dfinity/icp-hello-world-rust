FROM --platform=linux/amd64 rust:1.75-slim-bookworm

RUN apt -yq update 
RUN apt -yqq install --no-install-recommends curl ca-certificates libunwind-dev git

# Install dfx
RUN sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

# Install wasm32-unknown-unknown target
RUN rustup target add wasm32-unknown-unknown

# Install PocketIC
RUN curl -sLO https://download.dfinity.systems/ic/69e1408347723dbaa7a6cd2faa9b65c42abbe861/openssl-static-binaries/x86_64-linux/pocket-ic.gz
RUN gzip -d pocket-ic.gz
RUN chmod +x pocket-ic
