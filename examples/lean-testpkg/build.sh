# This script manually builds lean-testpkg so we can compare.

#!/bin/sh
set -ex

LEAN_TOOLCHAIN="$HOME/.elan/toolchains/leanprover-lean4-nightly"

LEAN="$LEAN_TOOLCHAIN/bin/lean"
LEANC="$LEAN_TOOLCHAIN/bin/leanc"

TMPC=lean-testpkg.c
BUILD="target/debug"

$LEAN -c $TMPC src/main.lean

mkdir -p $BUILD
$LEANC -o $BUILD/lean-testpkg $TMPC

rm $TMPC