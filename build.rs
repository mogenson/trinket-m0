use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

extern crate bindgen;
extern crate cc;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");

    let defines = ["__SAMD21E18A__"];

    let flags = [
        "-ffunction-sections",
        "-fno-pic",
        "-nostartfiles",
        "-g3",
        "-mcpu=cortex-m0plus",
        "-mlong-calls",
        "-mthumb",
        "-Os",
        "-std=gnu99",
        "-Wall",
    ];

    let includes = [
        "atmel-start",
        "atmel-start/CMSIS/Core/Include",
        "atmel-start/config",
        "atmel-start/hal/include",
        "atmel-start/hal/utils/include",
        "atmel-start/hpl/core",
        "atmel-start/hpl/dmac",
        "atmel-start/hpl/gclk",
        "atmel-start/hpl/pm",
        "atmel-start/hpl/port",
        "atmel-start/hpl/sysctrl",
        "atmel-start/hri",
        "atmel-start/samd21a/include",
    ];

    // exclude main.c, startup_samd21.c, driver_examples.c
    let files = [
        "atmel-start/atmel_start.c",
        "atmel-start/driver_init.c",
        "atmel-start/hal/src/hal_atomic.c",
        "atmel-start/hal/src/hal_delay.c",
        "atmel-start/hal/src/hal_gpio.c",
        "atmel-start/hal/src/hal_init.c",
        "atmel-start/hal/src/hal_io.c",
        "atmel-start/hal/src/hal_sleep.c",
        "atmel-start/hal/utils/src/utils_assert.c",
        "atmel-start/hal/utils/src/utils_event.c",
        "atmel-start/hal/utils/src/utils_list.c",
        "atmel-start/hal/utils/src/utils_syscalls.c",
        "atmel-start/hpl/core/hpl_core_m0plus_base.c",
        "atmel-start/hpl/core/hpl_init.c",
        "atmel-start/hpl/dmac/hpl_dmac.c",
        "atmel-start/hpl/gclk/hpl_gclk.c",
        "atmel-start/hpl/pm/hpl_pm.c",
        "atmel-start/hpl/sysctrl/hpl_sysctrl.c",
        "atmel-start/samd21a/gcc/system_samd21.c",
        "src/hal.c",
    ];

    let mut builder = cc::Build::new();
    builder.pic(false);
    builder.archiver("arm-none-eabi-ar");

    let mut bindings = bindgen::Builder::default();
    bindings = bindings.clang_arg("--sysroot=/usr/lib/gcc/arm-none-eabi/include");
    bindings = bindings.header("atmel-start/atmel_start.h");
    bindings = bindings.header("src/hal.h");
    bindings = bindings.ctypes_prefix("cty");
    bindings = bindings.use_core();
    bindings = bindings.trust_clang_mangling(false);

    // add defines
    for define in defines.iter() {
        builder.define(define, None);
        bindings = bindings.clang_arg(format!("-D{}", define));
    }

    // add compiler flags
    for flag in flags.iter() {
        builder.flag(flag);
    }

    // add include paths
    for include in includes.iter() {
        builder.include(include);
        bindings = bindings.clang_arg(format!("-I{}", include));
    }

    // add source files, rebuild if modified
    for file in files.iter() {
        builder.file(file);
        println!("cargo:rerun-if-changed={}", file);
    }

    // compile the atmel-start sources as a static library
    builder.compile("libhal.a");

    // write bindings to file
    bindings
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
