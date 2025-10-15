# aarch64_bootloader

Minimal AArch64 bootloader to load my [kernel](https://github.com/yoshipep/aarch64_kernel), for learning purposes.

This project is part of a blog series I’m writing at [jcomes.org](https://jcomes.org/), where I document the process of writing a simple OS for the AArch64 architecture — using Rust (a language I’m learning along the way). The goal is to go from boot to a minimal working kernel, exploring memory management, exception handling, and more.

## 🔧 Requirements

You'll need:

- A Linux environment (Ubuntu 22.04+ recommended)
- Rust (via [rustup.rs](https://rustup.rs))
- AArch64 cross-compilation tools (`gcc-aarch64-linux-gnu` or `binutils` + `gcc` built manually)

Install required tools via apt:

```bash
sudo apt update
sudo apt install qemu-system-aarch64 gcc-14-aarch64-linux-gnu binutils-aarch64-linux-gnu
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add aarch64-unknown-none
```

🚀 Build & Run

To compile and run:

```bash
make run
```

