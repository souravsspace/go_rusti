#  Build stage: compile the binary with glibc
FROM rust:1.85 AS builder

WORKDIR /usr/src/go_rusti

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
	cargo fetch

# Copy source code
COPY . .

# Build in release mode
RUN cargo build --release


# 2. Runtime stage: lightweight Debian
FROM debian:bookworm-slim

# Install CA certificates and shell
RUN apt-get update && \
	apt-get install -y --no-install-recommends ca-certificates bash && \
	rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the compiled binary
COPY --from=builder /usr/src/go_rusti/target/release/go_rusti /app/go_rusti
RUN chmod +x /app/go_rusti

# Expose port if you use start_node
EXPOSE 8000

# Mount data directory
VOLUME ["/app/data/blocks"]

# Start an interactive shell by default
ENTRYPOINT ["/bin/bash", "-l"]
CMD ["-c", "echo 'Welcome to go_rusti container! Use /app/go_rusti <cmd>'; exec /bin/bash -l"]
