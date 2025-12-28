#!/usr/bin/env bash

sudo pacman -Rns cmdcreate cmdcreate-debug --noconfirm || clear

echo -e "\nRelease ver not installed, skipping\n"

rustup update

cargo update

./package/format.sh

cargo build --release

cargo clippy --fix --allow-no-vcs --release

sudo cp ./target/release/cmdcreate /usr/bin
