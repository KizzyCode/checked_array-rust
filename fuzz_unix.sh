#!/bin/sh
set -eu

# Get the amount of cores or fallback to one
CORES=`getconf _NPROCESSORS_ONLN || echo 1`

# Start the fuzzing
cargo +nightly fuzz run --jobs $CORES --sanitizer none checked_array_fuzz