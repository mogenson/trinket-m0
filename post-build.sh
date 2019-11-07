#!/bin/sh

name=trinket-m0

# build binary
arm-none-eabi-objcopy target/thumbv6m-none-eabi/debug/$name -O binary $name.bin

# make UF2
uf2conv-rs -b 0x2000 -f 0x68ed2b88 -o $name.uf2 $name.bin
