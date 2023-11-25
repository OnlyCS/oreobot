#!/bin/bash

cargo build --release --bin oreo-cache --features=bin
cp ../target/release/oreo-cache ./oreo-cache