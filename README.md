<p align="center">
<img src="docs/site/static/img/logo.svg" alt="Logo" width="100" height="100">
</p>

# Haneul

Haneul Core implements a decentralized, programmable distributed ledger which provides a digital infrastructure that can empower billions of people.

[![Github release](https://img.shields.io/github/v/release/GeunhwaJeong/haneul.svg?sort=semver)](https://github.com/GeunhwaJeong/haneul/releases/latest)
[![License](https://img.shields.io/github/license/GeunhwaJeong/haneul)](https://github.com/GeunhwaJeong/haneul/blob/main/LICENSE)

## Building from Source

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The required Rust version is managed automatically via [`rust-toolchain.toml`](rust-toolchain.toml).

### 2. Install Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update && sudo apt-get install -y \
    build-essential libssl-dev pkg-config libclang-dev cmake protobuf-compiler
```

**macOS:**
```bash
brew install cmake protobuf
```

### 3. Build

```bash
git clone https://github.com/GeunhwaJeong/haneul.git
cd haneul
cargo build --release
```

## Running a Local Node

```bash
# Start a local validator with faucet
./target/release/haneul start --with-faucet --force-regenesis

# Switch to local environment
./target/release/haneul client switch --env local

# Get HANEUL tokens from faucet
./target/release/haneul client faucet

# Check balance
./target/release/haneul client gas
```

## Testing

```bash
# Unit tests
HANEUL_SKIP_SIMTESTS=1 cargo nextest run

# Test specific crate
cargo nextest run -p haneul-core

# Simulation tests
cargo simtest -p haneul-e2e-tests
```

## Linting

```bash
cargo fmt --all
cargo xclippy
```

## Project Structure

```
haneul/
├── crates/                    # Core Rust crates (haneul-core, haneul-node, haneul-types, ...)
├── consensus/                 # Mysticeti consensus engine
├── haneul-execution/          # Move VM execution layer
├── external-crates/           # Move compiler and VM
└── bridge/                    # Cross-chain bridge
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

Apache-2.0. See [LICENSE](LICENSE) for details.

This project is originally derived from [Sui](https://github.com/MystenLabs/sui) by [Mysten Labs](https://mystenlabs.com), licensed under Apache-2.0.
