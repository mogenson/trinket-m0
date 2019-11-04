#!/bin/sh

offset=0x2000
name=demo

# build binary
arm-none-eabi-objcopy target/thumbv6m-none-eabi/debug/$name -O binary $name.bin

if [ -f /dev/ttyACM0 ]; then
    # upload
    bossac --write --verify --reset --offset $offset $name.bin
else
    # make UF2
    uf2conv-rs -b $offset -f 0x68ed2b88 -o $name.uf2 $name.bin
fi
