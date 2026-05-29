#!/usr/bin/env bash

qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a72 \
    -smp 4 \
    -m 2048 \
    -bios ../firmware/QEMU_EFI.fd \
    -drive if=none,file=../nodes/pi-03.qcow2,id=hd0,format=qcow2 \
    -device virtio-blk-pci,drive=hd0 \
    \
    -netdev user,id=net0,hostfwd=tcp::2223-:22,hostfwd=tcp::7003-:7003 \
    -device virtio-net-pci,netdev=net0,mac=52:54:00:12:34:13 \
    \
    -serial mon:stdio \
    -nographic