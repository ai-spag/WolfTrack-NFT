FROM rust:1.72

# Install dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    build-essential \
    libudev-dev \
    git \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

# Install Solana CLI from source to /usr/local/bin
RUN git clone https://github.com/solana-labs/solana.git /solana && \
    cd /solana && \
    git checkout stable && \
    cargo build --release && \
    install -m 755 target/release/solana /usr/local/bin/solana && \
    install -m 755 target/release/solana-* /usr/local/bin/

# Install Anchor CLI globally and link to /usr/local/bin
RUN cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked && \
    ln -s /root/.cargo/bin/anchor /usr/local/bin/anchor

# Create workspace
WORKDIR /workspace
