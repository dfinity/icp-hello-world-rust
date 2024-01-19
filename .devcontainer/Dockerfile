FROM --platform=linux/amd64 rust:1.75-slim-bookworm

RUN apt -yq update 
RUN apt -yqq install --no-install-recommends curl ca-certificates libunwind-dev git

# Install dfx
RUN sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
