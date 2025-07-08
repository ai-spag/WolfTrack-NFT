FROM rust:1.72

# Install Solana CLI
RUN apt-get update && apt-get install -y curl pkg-config libssl-dev build-essential libudev-dev git && \
    sh -c "$(curl -sSfL https://release.solana.com/stable/install)" && \
    echo 'export PATH="/root/.local/share/solana/install/active_release/bin:$PATH"' >> /root/.bashrc

# Install Anchor
RUN cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Re-expose solana/anchor paths
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"
ENV PATH="/usr/local/cargo/bin:$PATH"

# Create workspace directory
WORKDIR /workspace
