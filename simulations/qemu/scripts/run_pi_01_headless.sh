#!/usr/bin/env bash

qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a72 \
    -smp 4 \
    -m 2048 \
    -bios ../firmware/QEMU_EFI.fd \
    -drive if=none,file=../nodes/pi-01.qcow2,id=hd0,format=qcow2 \
    -device virtio-blk-pci,drive=hd0 \
    -netdev user,id=net0,hostfwd=tcp::2221-:22 \
    -device virtio-net-pci,netdev=net0 \
    -serial mon:stdio \
    -nographic