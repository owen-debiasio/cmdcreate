#!/usr/bin/env bash

if [[ -z "$1" ]]; then
    echo "Provide cmdcreate's version"
    exit 1
fi

VERSION="$1"

cargo update

rustup update

cargo build --release

cp ../target/release/cmdcreate ~/Downloads

mv ~/Downloads/cmdcreate ~/Downloads/cmdcreate-"$VERSION"-linux-x86_64-bin

echo -e "\nPackaged cmdcreate to \"cmdcreate-v$VERSION-linux-x86_64-bin\""
