#!/bin/bash

sudo chown -R "$(id -u)":"$(id -g)" .
cargo build --release
sudo chown -R 999:999 .
