#!/bin/sh

ELF="$1"
BIN="$ELF.bin"
UF2=$(basename "$ELF").uf2

if [ -z "$ELF" ] || [ ! -x "$ELF" ]; then
    echo "use 'cargo run --example <NAME>' to make UF2 file"
    exit 1
fi

echo "hijacking cargo runner to make UF2 file"

# print stats
arm-none-eabi-size "$ELF"

# build binary
arm-none-eabi-objcopy "$ELF" -O binary "$BIN"

# make UF2
uf2conv-rs -b 0x2000 -f 0x68ed2b88 -o "$UF2" "$BIN"

echo "wrote $UF2"
