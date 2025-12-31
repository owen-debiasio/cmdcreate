#!/usr/bin/env bash
 
set -e

sudo pacman -Rns cmdcreate cmdcreate-debug --noconfirm || clear

echo -e "\nRelease ver not installed, skipping\n"

rustup update
rustup default stable

cargo update

./package/format.sh

cargo build --release

cargo clippy --fix --allow-no-vcs --release

sudo cp ./target/release/cmdcreate /usr/bin
