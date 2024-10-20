#!/bin/bash

cargo build --release
wasm-bindgen --target web target/wasm32-unknown-unknown/release/topaz_web.wasm --out-dir ./www