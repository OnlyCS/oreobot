#!/bin/bash

cargo build --release --bin oreo-logger --features=bin
cp ../target/release/oreo-logger ./oreo-logger