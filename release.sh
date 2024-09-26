#!/bin/bash

# This script builds release binaries of the game,
# creates a release directory for a specific version,
# compresses the binaries to the directory
# and generates sha256 checksums to a checksums.txt file.
#
# Example of running this script: ./release.sh 0.1.0

set -e

if [ -z "$1" ]; then
	echo "Usage: ${} <version>" >> /dev/stderr
	exit 1
fi

VERSION=$1
BINARY_NAME="terminal_rpg"

echo "Moving to the game crate directory"
cd game

echo "Adding compilation targets..."
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu

echo "Building release binaries..."
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

OUTPUT_DIR="releases/$VERSION"
mkdir -p $OUTPUT_DIR
rm -rf $OUTPUT_DIR/*

echo "Compressing binaries"
LINUX_TAR="${OUTPUT_DIR}/${BINARY_NAME}_${VERSION}_linux.tar.gz"
WINDOWS_ZIP="${OUTPUT_DIR}/${BINARY_NAME}_${VERSION}_windows.zip"

tar -czvf $LINUX_TAR -C target/x86_64-unknown-linux-gnu/release $BINARY_NAME
zip $WINDOWS_ZIP -j target/x86_64-pc-windows-gnu/release/${BINARY_NAME}.exe

echo "Generating SHA256 checksums"
CHECKSUM_FILE="$OUTPUT_DIR/checksums.txt"
sha256sum $LINUX_TAR > $CHECKSUM_FILE
sha256sum $WINDOWS_ZIP >> $CHECKSUM_FILE

echo "Release files generated in $OUTPUT_DIR"
ls -l $OUTPUT_DIR
