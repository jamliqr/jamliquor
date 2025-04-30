# Building JamLiquor

## Cross-Compilation Support

JamLiquor supports cross-compilation for multiple architectures:

### Supported Architectures
- RISC-V (riscv64gc-unknown-linux-gnu)
- ARM (armv7-unknown-linux-gnueabihf)
- x86_64
- AArch64

### Prerequisites
- Rust toolchain (stable)
- Cross-compilation toolchains for target architectures

### Cross-Compilation Setup

1. Install target toolchains:
```bash
rustup target add riscv64gc-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
```

2. Build for specific architecture:
```bash
# RISC-V
cargo build --target riscv64gc-unknown-linux-gnu

# ARM
cargo build --target armv7-unknown-linux-gnueabihf
```

### Testing
Run cross-compilation tests in CI to ensure compatibility.
