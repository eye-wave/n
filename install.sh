#!/bin/bash

if [ "$(uname)" != "Linux" ]; then
  echo "Script not yet supported on other platforms"
  exit
fi

if [[ "$@" == *"--stable"* || "$@" == *"-s"* ]]; then
  cargo build --release
  cp target/release/ru ~/.local/bin
else
  cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release
  cp target/x86_64-unknown-linux-gnu/release/ru ~/.local/bin
fi
