#!/bin/bash
cd /home/claude/work/src
export RUSTC_BOOTSTRAP=1 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_OPT_LEVEL=s
export CARGO_TARGET_DIR=/home/claude/work/target CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++ AR_x86_64_pc_windows_gnu=x86_64-w64-mingw32-ar
cargo test --release --no-run --target x86_64-pc-windows-gnu -p mdm-app --bins --message-format=short 2>&1; cargo test --release --no-run --target x86_64-pc-windows-gnu -p mdm-tools -p mdm-extract -p mdm-engine -p mdm-ipc -p mdm-crash --lib --message-format=short 2>&1
