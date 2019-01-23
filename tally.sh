#!/bin/sh

export RUST_BACKTRACE=1
exec cargo run -- $@
