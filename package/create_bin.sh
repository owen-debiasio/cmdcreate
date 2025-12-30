#!/usr/bin/env bash

if [[ -z "$1" ]]; then
    echo "Provide cmdcreate's version"
    exit 1
fi

cargo update
rustup update

cargo build --release

cp ../target/release/cmdcreate ~/Downloads

mv ~/Downloads/cmdcreate ~/Downloads/cmdcreate-"$1"-linux-x86_64-bin

echo -e "\nPackaged cmdcreate to \"cmdcreate-v$1-linux-x86_64-bin\""
