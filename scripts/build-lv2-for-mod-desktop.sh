#!/bin/bash
binary_name="libdm_space_echo.dylib"
move_to="dm-SpaceEcho.lv2/$binary_name"

cd lv2
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
MACOSX_DEPLOYMENT_TARGET=10.15 cargo build --release --target x86_64-apple-darwin
MACOSX_DEPLOYMENT_TARGET=10.15 cargo build --release --target aarch64-apple-darwin
lipo -create target/x86_64-apple-darwin/release/$binary_name target/aarch64-apple-darwin/release/$binary_name -output target/release/$binary_name
file target/release/$binary_name

if [ -d "$move_to" ]; then
    rm -r "$move_to"
fi

if mv target/release/$binary_name $move_to; then
    echo "Copied lv2 binary to $move_to"
fi