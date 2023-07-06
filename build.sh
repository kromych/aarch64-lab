#!/bin/sh

TARGET_JSON="./lab/lab.json"
TARGET="lab"
CARGO_OPT="-Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem"

cargo clippy --target ${TARGET_JSON} ${CARGO_OPT} -p aarch64-lab
cargo build --release --target ${TARGET_JSON} ${CARGO_OPT} -p aarch64-lab
cargo build --target ${TARGET_JSON} ${CARGO_OPT} -p aarch64-lab

OBJCOPY=$(find `rustc --print sysroot` -name "llvm-objcopy")

${OBJCOPY} \
    -O binary -R .note -R .note.gnu.build-id -R .comment -S \
    target/${TARGET}/release/aarch64-lab \
    target/${TARGET}/release/aarch64-lab.Image

${OBJCOPY} \
    -O binary -R .note -R .note.gnu.build-id -R .comment -S \
    target/${TARGET}/debug/aarch64-lab \
    target/${TARGET}/debug/aarch64-lab.Image
