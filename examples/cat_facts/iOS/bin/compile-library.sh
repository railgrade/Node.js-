#!/usr/bin/env bash

if [ "$#" -ne 1 ]; then
  echo "Usage (note: only call inside XCode!):"
  echo "$0 <FFI_TARGET>"
  exit 1
fi

# what to pass to cargo build -p, e.g. your_lib_ffi
FFI_TARGET=$1

set -euvx

RELFLAG=
if [[ "$CONFIGURATION" != "Debug" ]]; then
  RELFLAG=--release
fi

IS_SIMULATOR=0
if [ "${LLVM_TARGET_TRIPLE_SUFFIX-}" =