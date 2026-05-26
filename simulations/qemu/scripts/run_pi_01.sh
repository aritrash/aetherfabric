#!/usr/bin/env bash

qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a72 \
    -smp 4 \
    -m 4096 \
    -bios ../firmware/QEMU_EFI.fd \
    -drive if=virtio,file=../nodes/pi-01.qcow2,format=qcow2 \
    -drive file=../base/ubuntu-26.04-live-server-arm64.iso,media=cdrom \
    -boot d \
    -netdev user,id=net0,hostfwd=tcp::2221-:22 \
    -device virtio-net-pci,netdev=net0 \
    -nographic