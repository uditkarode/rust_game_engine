#!/bin/bash
HOST="$(rustc -vV | grep host | awk '{print $2}')"
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
    --target $HOST --release