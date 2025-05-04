#!/bin/sh
#-trace "*" \

TARGET="lab"
FLAVOR="debug"
KERNEL="target/${TARGET}/${FLAVOR}/aarch64-lab.Image"

MACHINE="virt,gic-version=3,highmem=on,virtualization=off"
CPU="cortex-a76" # max # host

qemu-system-aarch64 -machine ${MACHINE} -machine dumpdtb=./dump.dtb
dtc -I dtb -O dts -o ./dump.dts ./dump.dtb

qemu-system-aarch64 \
    -nodefaults \
    -kernel ${KERNEL} \
    -machine ${MACHINE} -nographic -accel tcg -cpu ${CPU} \
    -smp 1 \
    -m 64M \
    --semihosting \
    -chardev stdio,id=char0,mux=on,logfile=serial1.log,signal=off \
    -serial chardev:char0 \
    -mon chardev=char0 \
    -chardev file,path=serial2.log,id=char1 \
    -serial chardev:char1 \
    -nographic \
    -s \
    --trace "gicv3_redist*" --trace "gicv3_icc*" --trace "gicv3_dist*" \
    -D qemu.log \
    # -S \
    # -d guest_errors -d cpu_reset -d int
