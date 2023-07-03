#!/bin/sh
#-trace "*" \

qemu-system-aarch64 \
    -nodefaults \
    -kernel target/lab/release/aarch64-lab.Image \
    -machine virt,highmem=off -nographic -accel tcg -cpu max \
    -smp 1 \
    -m 256M \
    --semihosting \
    -chardev stdio,id=char0,mux=on,logfile=serial1.log,signal=off \
    -serial chardev:char0 \
    -mon chardev=char0 \
    -chardev file,path=serial2.log,id=char1 \
    -serial chardev:char1 \
    -nographic \
#    -d guest_errors -d cpu_reset -d int -D qemu.log
