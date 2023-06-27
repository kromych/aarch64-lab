#!/bin/sh

qemu-system-aarch64 -kernel target/aarch64-unknown-linux-gnu/release/aarch64-lab.Image -machine virt,highmem=off -nographic -accel tcg -cpu cortex-a53