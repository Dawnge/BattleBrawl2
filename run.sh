#!/bin/bash
cargo build --release --target wasm32-unknown-unknown
cargo run --release --target x86_64-pc-windows-gnu
#./target/x86_64-pc-windows-gnu/debug/battle_brawl2.exe