#!/bin/sh

export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export OBJCOPY=llvm-objcopy
alias objcopy=$OBJCOPY

TARGET_JSON="./lab.json"
TARGET="lab"
CARGO_OPT="-Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem"

cargo clippy --target ${TARGET_JSON} ${CARGO_OPT}
cargo build --release --target ${TARGET_JSON} ${CARGO_OPT}
cargo build --target ${TARGET_JSON} ${CARGO_OPT}

objcopy \
    -O binary -R .note -R .note.gnu.build-id -R .comment -S \
    target/${TARGET}/release/aarch64-lab \
    target/${TARGET}/release/aarch64-lab.Image

objcopy \
    -O binary -R .note -R .note.gnu.build-id -R .comment -S \
    target/${TARGET}/debug/aarch64-lab \
    target/${TARGET}/debug/aarch64-lab.Image
