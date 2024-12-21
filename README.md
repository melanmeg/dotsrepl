# dotsrepl

## Develop

### Rust

```bash
# pre-setting
sudo apt install -y build-essential
cargo new develop && cd develop
cargo add clap --features derive

# simple run
cargo run -q -- -h
```

- Release

```bash
# 古いglibでも動くように配布する
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl

# arm用作成
sudo apt install -y gcc-aarch64-linux-gnu
rustup target add aarch64-unknown-linux-musl
cargo build --release --target=aarch64-unknown-linux-musl
RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" cargo build --release --target=aarch64-unknown-linux-musl
cp -a ./target/aarch64-unknown-linux-musl/release/dotfiles ../.bin/dotfiles_arm
```
