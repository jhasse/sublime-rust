#!/usr/bin/env bash
# Install/update rust.
# The first argument should be the toolchain to install.

set -ex
if [ -z "$1" ]
then
    echo "First parameter must be toolchain to install."
    exit 1
fi
TOOLCHAIN="$1"

if ! [ -x "$(command -v rustup)" ]
then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --default-toolchain none
    source $HOME/.cargo/env
fi
if [ "$(uname -s)" == "Linux" ] && ! [ -x "$(command -v cc)" ]
then
    apt-get update
    apt-get -y install --no-install-recommends build-essential
fi
rustup set profile minimal
rustup component remove --toolchain=$TOOLCHAIN rust-docs || echo "already removed"
rustup update --no-self-update $TOOLCHAIN
rustup default $TOOLCHAIN
rustup component add clippy
rustup component add rust-src
# cargo install cargo-script
rustup -V
rustc -Vv
cargo -V
