#!/bin/sh
sudo losetup -o1048576 /dev/loop2 hd.img
sudo mount /dev/loop2 /mnt

