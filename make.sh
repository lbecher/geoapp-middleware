#!/bin/bash

# instalando plataformas
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add powerpc64-unknown-linux-gnu
rustup target add riscv64gc-unknown-linux-gnu

# compilando
#cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
#cargo build --release --target x86_64-unknown-linux-gnu
#cargo build --release --target aarch64-unknown-linux-gnu
#cargo build --release --target powerpc64-unknown-linux-gnu
#cargo build --release --target riscv64gc-unknown-linux-gnu