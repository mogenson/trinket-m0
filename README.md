# Atmel START + Rust for Adafruit Trinket M0

## Overview

This repo is an experiment in combining C and Rust code for a Cortex M0 microcontroller. I construct a Rust hardware abstraction layer around the C hardware drivers from Atmel. This way I don't have to re-implement each hardware driver in Rust.

For example, I setup the samd21 to run at 48 MHz using the DFLL clock generator via the https://start.atmel.com website. This configuration is applied when `system_init()` is called, without having to manipulate the clock peripheral using Rust.

## Usage

Run `cargo build --example blinky` to generate a `blinky` ELF file starting at 0x2000 for a Trinket M0 board. Install [uf2conv-rs](https://github.com/sajattack/uf2conv-rs) and use `cargo run --example blinky` to generate a `blinky.uf2` file to copy to the mass storage device partition of a Trinket M0 in bootloader mode.

## How the build works

The `src/hal.c` and `src/hal.h` files implement some C delay and gpio functions that are wrapped in `unsafe` blocks and called from `src/lib.rs`. I implement the basic functionality needed for some of the `embedded_hal` traits.

The `build.rs` file collects the C source files, headers, and compiler flags from the generated `atmel-start` project. The C files (except `main.c` and the examples) are compiled into a static archive: `libhal.a`. This archive is linked to the Rust application. Function signatures, constants, and defines are extracted from the C header files so that Rust can call C functions.

## C startup

The current configuration allows `startup_samd21.c` to initialize the microcontroller hardware, including the vector table and exception handlers. I modify Rust's `main()` function to be callable from C. A `__libc_init_array()` shim function is included in `hal.c` to satisfy `libhal.a`'s dependency on `libc`. The `Reset_Handler` must be declared `EXTERN` in the linker in order to include the vector table from `libhal.a`.

## Rust startup

In the `rust-startup` branch, Rust sets up the default Cortex M0 vector table, the stack, and copies variables to from. Setting up interrupts will require populating the rest of the vector table with matching symbols from the C drivers.
