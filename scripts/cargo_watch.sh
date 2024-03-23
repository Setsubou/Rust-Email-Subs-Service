#!/usr/bin/env bash
# Script to start cargo watch
cd ..
cargo run watch -x check -x test -run
