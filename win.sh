#!/bin/sh

BIN_NAME='chess'

cargo build -p app --target x86_64-pc-windows-gnu \
    && cp -f target/x86_64-pc-windows-gnu/debug/$BIN_NAME.exe . \
    && exec ./$BIN_NAME.exe "$@"
