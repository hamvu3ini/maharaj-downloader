#!/bin/bash
# Windows build of Maharaj Downloader (cross-compiled, std built from source)
cd /home/claude/work/src
export RUSTC_BOOTSTRAP=1
export CARGO_PROFILE_RELEASE_LTO=off
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
export CARGO_PROFILE_RELEASE_DEBUG=0
export CARGO_PROFILE_RELEASE_OPT_LEVEL=s
export CARGO_TARGET_DIR=/home/claude/work/target
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++
export AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar
cargo build --release --target x86_64-pc-windows-gnu -p mdm-app -p mdm-nmhost -p mdm-cli -p mdm-mcp "$@"
