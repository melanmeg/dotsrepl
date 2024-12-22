# dotfiles replace

## これは何？

- 現在のディレクトリのドットファイルをホームディレクトリに設置する
- その際にバックアップも取れる

## インストール

```bash
PACKAGE_NAME=dotsrepl
sudo curl -Lo "$HOME/.local/bin/$PACKAGE_NAME" https://github.com/melanmeg/dotsrepl/releases/download/v1.0/dotsrepl-1.0-x86_64-unknown-linux-musl
sudo chown "$USER:$USER" "$HOME/.local/bin/$PACKAGE_NAME"
sudo chmod +x "$HOME/.local/bin/$PACKAGE_NAME"
```

- armはこちら `https://github.com/melanmeg/dotsrepl/releases/download/v1.0/dotsrepl-1.0-aarch64-unknown-linux-musl`

## 使い方

```bash
$ dotsrepl -h
Rust CLI for Dotfiles.

Usage: dotsrepl [OPTIONS]

Options:
  -p, --path <PATH>       Path to dotfiles
  -f, --force             Force overwrite
  -b, --backup            Backup dotfiles
  -l, --link              Link to homedir
  -g, --gitconfig-shared  git config (.gitconfig_shared)
  -h, --help              Print help
  -V, --version           Print version
```

```bash
$ dotsrepl -p . -bg
install dotfiles to homedir...

Backup file already exists at /home/melanmeg/.dotbackup. Removing it...

backup dotfiles...
/home/melanmeg/.dotbackup not found. Auto Make it.

Backing up /home/melanmeg/.bin...
Backing up /home/melanmeg/.vimrc...
Backing up /home/melanmeg/.colorrc...
Backing up /home/melanmeg/.zshrc...
Backing up /home/melanmeg/.zpreztorc...
Backing up /home/melanmeg/.mise.toml...
Backing up /home/melanmeg/.vscode...
Backing up /home/melanmeg/.env.sample...
Backing up /home/melanmeg/.gitconfig_shared...
Backing up /home/melanmeg/.gitignore_global...
Backing up /home/melanmeg/.bashrc...
Backing up /home/melanmeg/.vim...
Backing up /home/melanmeg/.my...
copy to homedir /home/melanmeg/.bin...
copy to homedir /home/melanmeg/.vimrc...
copy to homedir /home/melanmeg/.colorrc...
copy to homedir /home/melanmeg/.zshrc...
copy to homedir /home/melanmeg/.zpreztorc...
copy to homedir /home/melanmeg/.mise.toml...
copy to homedir /home/melanmeg/.vscode...
copy to homedir /home/melanmeg/.env.sample...
copy to homedir /home/melanmeg/.gitconfig_shared...
copy to homedir /home/melanmeg/.gitignore_global...
copy to homedir /home/melanmeg/.bashrc...
copy to homedir /home/melanmeg/.vim...
copy to homedir /home/melanmeg/.my...

command execute: git config --global include.path ~/.gitconfig_shared


 Install completed!!!!

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
