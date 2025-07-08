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

# Install Solana CLI from source
RUN git clone https://github.com/solana-labs/solana.git /solana && \
    cd /solana && \
    git checkout stable && \
    cargo build --release && \
    cp target/release/solana* /usr/local/bin/

# Install Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked && \
    cp /root/.cargo/bin/anchor /usr/local/bin/

# Set working directory
WORKDIR /workspace
