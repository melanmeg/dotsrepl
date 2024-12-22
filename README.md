# dotsrepl

## これは何？

- 現在のディレクトリのドットファイルをホームディレクトリに設置する
- その際にバックアップも取れる

## インストール

```bash
PACKAGE_NAME=dotsrepl
sudo curl -Lo "$HOME/.local/bin/$PACKAGE_NAME" https://github.com/melanmeg/dotsrepl/releases/download/v1.0/dotfiles-1.0-x86_64-unknown-linux-musl
sudo chown "$USER:$USER" "$HOME/.local/bin/$PACKAGE_NAME"
sudo chmod +x "$HOME/.local/bin/$PACKAGE_NAME"
```

## 使い方

```bash
$ dotsrepl -h
Rust CLI for Dotfiles.

Usage: dotsrepl [OPTIONS]

Options:
  -f, --force             Force overwrite
  -b, --backup            Backup dotfiles
  -l, --link              Link to homedir
  -g, --gitconfig-shared  git config (.gitconfig_shared)
  -h, --help              Print help
  -V, --version           Print version
```

```bash

```

## 開発

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
RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" cargo build --release --target=aarch64-unknown-linux-musl
```
