#!/bin/bash
set -e
cd $(dirname "$0")

SRC_DIR=$(dirname $(rustup which rustc))"/../lib/rustlib/src/rust/"
DST_DIR="sysroot_src"

if [ ! -e $SRC_DIR ]; then
    echo "Please install rust-src component"
    exit 1
fi

rm -rf $DST_DIR
mkdir -p $DST_DIR/src
cp -r $SRC_DIR/src $DST_DIR/

echo "Successfully prepared libcore for building"
