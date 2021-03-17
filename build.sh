#!/bin/bash
cargo build --target x86_64-unknown-linux-musl --release
strip target/x86_64-unknown-linux-musl/release/casscf-tete -o casscf-tete
