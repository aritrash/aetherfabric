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
    -netdev socket,id=cluster1,listen=:12345 \
    -device virtio-net-pci,netdev=cluster1 \
    -netdev socket,id=cluster2,listen=:12346 \
    -device virtio-net-pci,netdev=cluster2 \
    -serial mon:stdio \
    -nographic