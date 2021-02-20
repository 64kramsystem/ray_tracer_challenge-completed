#!/bin/sh

set -o errexit

cargo build --release --target=riscv64gc-unknown-linux-gnu "$@"
