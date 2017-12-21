#!/bin/bash

wget https://github.com/c4milo/github-release/releases/download/v1.1.0/github-release_v1.1.0_linux_amd64.tar.gz
tar zxvf github-release_v1.1.0_linux_amd64.tar.gz

./github-release SilentCircle/scaelc $CI_COMMIT_TAG master "Release $CI_COMMIT_TAG" target/x86_64-unknown-linux-musl/release/sc-audit-event-log-collector
rm github-release*
