#! /bin/bash

rm -rf ~/.local/bin/i3-eww

cargo build

cp ./target/release/i3-eww ~/.local/bin/i3-eww
