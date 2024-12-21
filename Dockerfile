FROM ubuntu:24.04

# Init Setup
RUN sed -i.bak -r 's!http://(security|archive|[a-z]{2}\.archive).ubuntu.com/ubuntu!http://ftp.riken.go.jp/Linux/ubuntu!' /etc/apt/sources.list.d/ubuntu.sources
RUN apt upgrade -yU
RUN apt purge -y needrestart
RUN apt install -y git vim curl wget sudo
RUN echo "set bell-style none" | tee -a /etc/inputrc
RUN chmod -x /etc/update-motd.d/*

# Dockerにcodeが入らなかったため.（そもそも不要まである）
# RUN curl -Lo /tmp/code.deb https://vscode.download.prss.microsoft.com/dbazure/download/stable/f1e16e1e6214d7c44d078b1f0607b2388f29d729/code_1.91.1-1720564633_amd64.deb
# RUN apt install -y /tmp/code.deb
# RUN rm -f /tmp/code.deb

# Init User
RUN useradd -m myuser
RUN usermod -aG sudo myuser
RUN echo 'myuser ALL=(ALL) NOPASSWD:ALL' | tee /etc/sudoers.d/myuser

# myuser
USER myuser
WORKDIR /home/myuser

# git clone
ARG CACHEBURST=1
RUN git clone https://github.com/melanmeg/dotfiles.git

# Install shell
# Dockerだとインストールに失敗するパッケージは除外.
RUN sed -i '/ffmpeg\|gcloud\|azure-cli\|perl\|ansible-base/s/^/#/' ./dotfiles/.mise.toml
RUN USER=myuser HOME=/home/myuser \
  ./dotfiles/.bin/dotfiles -b
RUN USER=myuser HOME=/home/myuser \
  ./dotfiles/.bin/setup-libs.sh
RUN USER=myuser HOME=/home/myuser \
  ./dotfiles/.bin/install-golang.sh
RUN USER=myuser HOME=/home/myuser \
  ./dotfiles/.bin/install-clis.sh
# Dockerにcodeが入らなかったため.（そもそも不要まである）
# RUN USER=myuser HOME=/home/myuser \
#   ./dotfiles/.bin/install-recommended-vscode-extensions.sh
RUN USER=myuser HOME=/home/myuser \
  ./dotfiles/.bin/setup-zsh.sh

# After zsh comands
# $ zsh
# $ peco --version