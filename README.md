# Simple blockchain example

A simple UTXO-based blockchain implemented in Rust. This CLI application supports creating a blockchain, printing the chain, querying balances, sending tokens, and running a node server.

## Table of Contents

-  [Features](#features)
-  [Prerequisites](#prerequisites)
-  [Quick Start (Local)](#quick-start-local)
-  [CLI Commands](#cli-commands)
-  [Docker Usage](#docker-usage)
-  [Project Structure](#project-structure)
-  [License](#license)

## Features

-  Create a new blockchain with a coinbase genesis block
-  UTXO-based transaction model
-  Persistent storage using sled
-  Proof-of-work mining
-  CLI interface via Clap
-  Optional `--mine` flag to mine immediately
-  Running as a simple node server

## Prerequisites

-  [Rust](https://www.rust-lang.org/tools/install) (1.85+)
-  [Docker](https://docs.docker.com/get-docker/) (optional, for containerized usage)

## Quick Start (Local)

```bash
# Clone the repo
git clone https://github.com/yourusername/go_rusti.git
cd go_rusti

# Build and run:
cargo build --release
target/release/go_rusti --help
```

## CLI Commands

Use the built binary or `cargo run` for development.

```bash
# Initialize blockchain (run once)
cargo run create <ADDRESS>

# Print full chain
cargo run print_chain

# Check balance
cargo run get_balance <ADDRESS>

# Send tokens
cargo run send <FROM> <TO> <AMOUNT>

# Mine immediately
cargo run send <FROM> <TO> <AMOUNT> --mine

# Start node server
cargo run start_node <PORT> <ADDRESS>

# Help & version
cargo run --help
cargo run --version
```

## Docker Usage

1. **Build image**

   ```bash
   docker build -t go_rusti\:interactive .
   ```

2. **Run container**

   ```bash
   docker run -it --rm \
   -v $(pwd)/data/blocks:/app/data/blocks \
   go_rusti:interactive
   ```

3. **Inside container**, invoke commands:

   ```bash
   /app/go_rusti create <ADDRESS>
   /app/go_rusti print_chain
   /app/go_rusti get_balance <ADDRESS>
   /app/go_rusti send <FROM> <TO> <AMOUNT>
   /app/go_rusti send <FROM> <TO> <AMOUNT> --mine
   /app/go_rusti start_node <PORT> <ADDRESS>
   /app/go_rusti --help
   ```
