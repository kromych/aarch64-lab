#!/bin/sh

TARGET_JSON="./src/lab.json"
TARGET="lab"
CARGO_OPT="-Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem"

cargo clippy --target ${TARGET_JSON} ${CARGO_OPT}
cargo build --release --target ${TARGET_JSON} ${CARGO_OPT}
cargo build --target ${TARGET_JSON} ${CARGO_OPT}

OBJCOPY=$(find `rustc --print sysroot` -name "llvm-objcopy")

${OBJCOPY} \
    -O binary -R .note -R .note.gnu.build-id -R .comment -S \
    target/${TARGET}/release/aarch64-lab \
    target/${TARGET}/release/aarch64-lab.Image

${OBJCOPY} \
    -O binary -R .note -R .note.gnu.build-id -R .comment -S \
    target/${TARGET}/debug/aarch64-lab \
    target/${TARGET}/debug/aarch64-lab.Image
