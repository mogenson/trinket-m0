# Atmel START + Rust for Adafruit Trinket M0

## Overview

This repo is an experiment in combining C and Rust code for a Cortex M0 microcontroller. I construct a Rust hardware abstraction layer around the C hardware drivers from Atmel. This way I don't have to re-implement each hardware driver in Rust.

For example, I setup the samd21 to run at 48 MHz using the DFLL clock generator via the https://start.atmel.com website. This configuration is applied when `system_init()` is called, without having to manipulate the clock peripheral using Rust.

## Usage

Run `cargo build --example blinky` to generate a `blinky` ELF file starting at 0x2000 for a Trinket M0 board. Install [uf2conv-rs](https://github.com/sajattack/uf2conv-rs) and [cargo-make](https://github.com/sagiegurari/cargo-make) then use `cargo make uf2 blinky` to generate a `blinky.uf2` file to copy to the mass storage device partition of a Trinket M0 in bootloader mode.

## Examples

There are a few different examples demonstrating how to integrate a Rust application with the trinket-m0 library.

### [blinky.rs](https://github.com/mogenson/trinket-m0/blob/master/examples/blinky.rs)

Use the `embedded_hal` delay and digital traits to pulse the onboard red LED in a heartbeat pattern.

### [echo.rs](https://github.com/mogenson/trinket-m0/blob/master/examples/echo.rs)

Enumerate as a USB serial port and echo back any received ascii characters, but converted to uppercase. This example uses the C callback based API for the USB peripheral. All USB communication is interrupt driven so the main loop blinks an LED.

### [swirly.rs](https://github.com/mogenson/trinket-m0/blob/master/examples/swirly.rs)

Iterate through a range of hues and display the color on the built-in apa102 RGB LED. This example uses the `embedded_hal` write trait for SPI in conjunction with the [apa102-spi](https://github.com/smart-leds-rs/apa102-spi-rs) and [smart-leds](https://github.com/smart-leds-rs/smart-leds) drivers.

### [wiz.rs](https://github.com/mogenson/trinket-m0/blob/master/examples/wiz.rs)

Emulate the protocol used by a [Philips Wiz](https://www.usa.lighting.philips.com/consumer/smart-wifi-led) RGB light bulb by reading a JSON string over USB and setting the color of the onboard apa102 LED. This example combines a number of Rust components: ring buffers, heapless vectors, and deserialization of structs and enums with the C hardware abstraction library. You can test this example by redirecting the output of this Rust [lightbulb](https://github.com/mogenson/lightbulb) project to the USB serial port via: `./lightbulb > /dev/ttyACM0`.

## How the build works

The `src/hal.c` and `src/hal.h` files implement some C delay and gpio functions that are wrapped in `unsafe` blocks and called from `src/lib.rs`. I implement the basic functionality needed for some of the `embedded_hal` traits.

The `build.rs` file collects the C source files, headers, and compiler flags from the generated `atmel-start` project. The C files (except `main.c` and the examples) are compiled into a static archive: `libhal.a`. This archive is linked to the Rust application. Function signatures, constants, and defines are extracted from the C header files so that Rust can call C functions.

## C startup

The current configuration allows `startup_samd21.c` to initialize the microcontroller hardware, including the vector table and exception handlers. I modify Rust's `main()` function to be callable from C. A `__libc_init_array()` shim function is included in `hal.c` to satisfy `libhal.a`'s dependency on `libc`. The `Reset_Handler` symbol must be declared undefined in the linker invocation in order to include the vector table from `libhal.a`.

## Rust startup

In the `rust-startup` branch, Rust sets up the default Cortex M0 vector table, the stack, and copies variables to from. Setting up interrupts will require populating the rest of the vector table with matching symbols from the C drivers.

## Usefulness

The community at [atsamd-rs](https://github.com/atsamd-rs/atsamd) has been doing a great job of building Rust support for the samd21 microcontroller used in the Trinket M0. A pure Rust solution is the ideal method for writing an embedded application for this hardware. This project is simply an experiment to determine the feasibility of building a Rust application on top of a vendor provided C hardware abstraction library. It should not be considered production ready code.
