#!/bin/sh
#-trace "*" \

TARGET="lab"
FLAVOR="debug"
KERNEL="target/${TARGET}/${FLAVOR}/aarch64-lab.Image"

qemu-system-aarch64 \
    -nodefaults \
    -kernel ${KERNEL} \
    -machine virt,highmem=off -nographic -accel hvf -cpu host \
    -smp 1 \
    -m 256M \
    --semihosting \
    -chardev stdio,id=char0,mux=on,logfile=serial1.log,signal=off \
    -serial chardev:char0 \
    -mon chardev=char0 \
    -chardev file,path=serial2.log,id=char1 \
    -serial chardev:char1 \
    -nographic \
#    -s -S \
#    -d guest_errors -d cpu_reset -d int -D qemu.log
