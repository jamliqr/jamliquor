# JamLiquor

[← Back to Index](./index.md)

## Table of Contents
- [Quick Start](#-quick-start)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#-usage)
  - [Basic Usage](#basic-usage)
  - [Configuration](#configuration)
- [Architecture](#-architecture)
- [Development](#-development)
  - [Setting Up Development Environment](#setting-up-development-environment)
  - [Running Tests](#running-tests)
  - [Code Style](#code-style)
- [Contributing](#-contributing)
- [License](#-license)
- [Additional Resources](#-additional-resources)
- [Troubleshooting](#-troubleshooting)
  - [Common Issues](#common-issues)

JamLiquor is a post-quantum secure, cleanroom implementation of the JAM Protocol runtime environment, designed for efficient and verifiable execution of JAM-compatible code.

## 🚀 Quick Start

### Prerequisites
- Rust 1.75.0 or later
- Cargo
- Git
- Linux/Unix environment (Windows support via WSL2)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/jamliquor.git
cd jamliquor
```

2. Build from source:
```bash
cargo build --release
```

3. Run tests:
```bash
cargo test --all-features
```

## 📖 Usage

### Basic Usage
```bash
# Start the JAM runtime
cargo run --release -- run

# Validate a JAM block
cargo run --release -- validate <block-hash>

# Check runtime state
cargo run --release -- state info
```

### Configuration

Configuration is handled through either:
- Environment variables
- TOML configuration file (`config.toml`)
- Command-line arguments

Example `config.toml`:
```toml
[runtime]
max_block_weight = 1000000
max_block_length = 5242880
execution_timeout = 2000

[network]
listen_addr = "127.0.0.1"
listen_port = 30333
bootstrap_nodes = []
```

## 🏗️ Architecture

JamLiquor follows a modular architecture:

- `core/`: Core runtime implementation
- `crypto/`: Post-quantum cryptographic primitives
- `network/`: P2P networking stack
- `execution/`: JAM execution environment
- `validation/`: Block and transaction validation

For detailed architecture information, see [ARCHITECTURE.md](ARCHITECTURE.md).

## 🧪 Development

### Setting Up Development Environment

1. Install development dependencies:
```bash
# Install additional tools
cargo install cargo-watch cargo-audit cargo-tarpaulin

# Set up git hooks
./scripts/setup-dev.sh
```

2. Start development server:
```bash
cargo watch -x run
```

### Running Tests
```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test '*' --features integration

# Run specific test
cargo test test_name
```

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use clippy for linting (`cargo clippy`)
- Document public APIs
- Write tests for new features

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CLEANROOM.md#contributing) for details on:
- Code of Conduct
- Development Process
- Pull Request Guidelines
- Code Review Process

## 📄 License

JamLiquor is licensed under GPL-3.0. See [LICENSE](../LICENSE) for details.

## 🔗 Additional Resources

- [Project Manifesto](MANIFESTO.md)
- [Development Guidelines](CLEANROOM.md)
- [Milestones & Roadmap](MILESTONE.md)
- [Post-Quantum Cryptography](PQC.md)

## ❓ Troubleshooting

### Common Issues

1. **Build Failures**
   - Ensure Rust is up to date (`rustup update`)
   - Clear cargo cache (`cargo clean`)
   - Check for system dependencies

2. **Runtime Errors**
   - Verify configuration file
   - Check logs in `logs/jamliquor.log`
   - Ensure sufficient system resources

3. **Network Issues**
   - Check firewall settings
   - Verify network configuration
   - Ensure bootstrap nodes are accessible

For more help, please [open an issue](https://github.com/yourusername/jamliquor/issues).

---

**Navigation**
Previous: [Documentation Home](./index.md) | Next: [Project Manifesto](./MANIFESTO.md)
