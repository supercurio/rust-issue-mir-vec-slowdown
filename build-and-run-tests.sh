#!/bin/sh

RUSTC_VERSION=`rustc --version | cut -d" " -f 2`

TARGET_DIR="target"
if [ ! -z ${CARGO_TARGET_DIR} ]; then
	TARGET_DIR=${CARGO_TARGET_DIR}
fi

build_and_run() {
	if [ ! -z ${1} ]; then
		features="--features $1"
		echo "\nBuild with feature: $1\n"
	fi

	cargo build --release $features && \
	"./$TARGET_DIR/release/vec-mir-slowdown-issue"
}

build_and_run_all() {
	build_and_run
	build_and_run resize
	build_and_run extend_from_slice
}

build_and_run_all

if [ "$RUSTC_VERSION" = "1.12.0" ] || [ "$RUSTC_VERSION" = "1.12.1" ]; then
	echo "Now run again with 1.12.x MIR disabled"

	export RUSTFLAGS="-Zorbit=off"
	build_and_run_all
fi
