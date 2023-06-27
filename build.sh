#!/bin/sh

export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"
export CC=clang
export CXX=clang++
export LD=ld.lld
export AR=llvm-ar
export RANLIB=llvm-ranlib
export OBJCOPY=llvm-objcopy
export NM=llvm-nm
export TRIPLE=aarch64-unknown-linux-gnu
alias cc=$CC
alias c++=$CXX
alias ld=$LD
alias ar=$AR
alias ranlib=$RANLIB
alias objcopy=$OBJCOPY
alias nm=$NM

cargo clippy --target aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

objcopy \
    -O binary -R .note -R .note.gnu.build-id -R .comment -S \
    target/aarch64-unknown-linux-gnu/release/aarch64-lab \
    target/aarch64-unknown-linux-gnu/release/aarch64-lab.Image
