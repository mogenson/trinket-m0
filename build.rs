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

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");

    // TODO file list (exclude startup and main)

    // compile the atmel-start sources as a static library
    cc::Build::new()
        .pic(false)
        .define("__SAMD21E18A__", None)
        .flag("-ffunction-sections")
        .flag("-fno-pic")
        .flag("-nostartfiles")
        .flag("-g3")
        .flag("-mcpu=cortex-m0plus")
        .flag("-mlong-calls")
        .flag("-mthumb")
        .flag("-Os")
        .flag("-std=gnu99")
        .flag("-Wall")
        .include("atmel-start")
        .include("atmel-start/CMSIS/Core/Include")
        .include("atmel-start/config")
        .include("atmel-start/hal/include")
        .include("atmel-start/hal/utils/include")
        .include("atmel-start/hpl/core")
        .include("atmel-start/hpl/dmac")
        .include("atmel-start/hpl/gclk")
        .include("atmel-start/hpl/pm")
        .include("atmel-start/hpl/port")
        .include("atmel-start/hpl/sysctrl")
        .include("atmel-start/hri")
        .include("atmel-start/samd21a/include")
        .file("atmel-start/atmel_start.c")
        .file("atmel-start/driver_init.c")
        .file("atmel-start/examples/driver_examples.c")
        .file("atmel-start/hal/src/hal_atomic.c")
        .file("atmel-start/hal/src/hal_delay.c")
        .file("atmel-start/hal/src/hal_gpio.c")
        .file("atmel-start/hal/src/hal_init.c")
        .file("atmel-start/hal/src/hal_io.c")
        .file("atmel-start/hal/src/hal_sleep.c")
        .file("atmel-start/hal/utils/src/utils_assert.c")
        .file("atmel-start/hal/utils/src/utils_event.c")
        .file("atmel-start/hal/utils/src/utils_list.c")
        .file("atmel-start/hal/utils/src/utils_syscalls.c")
        .file("atmel-start/hpl/core/hpl_core_m0plus_base.c")
        .file("atmel-start/hpl/core/hpl_init.c")
        .file("atmel-start/hpl/dmac/hpl_dmac.c")
        .file("atmel-start/hpl/gclk/hpl_gclk.c")
        .file("atmel-start/hpl/pm/hpl_pm.c")
        .file("atmel-start/hpl/sysctrl/hpl_sysctrl.c")
        //.file("atmel-start/samd21a/gcc/gcc/startup_samd21.c")
        .file("atmel-start/samd21a/gcc/system_samd21.c")
        .file("src/hal.c")
        .archiver("arm-none-eabi-ar")
        .compile("libhal.a");

    let bindings = bindgen::Builder::default()
        .clang_arg("--sysroot=/usr/lib/gcc/arm-none-eabi/include")
        .clang_arg("-D__SAMD21E18A__")
        .clang_arg("-Iatmel-start")
        .clang_arg("-Iatmel-start/CMSIS/Core/Include")
        .clang_arg("-Iatmel-start/config")
        .clang_arg("-Iatmel-start/hal/include")
        .clang_arg("-Iatmel-start/hal/utils/include")
        .clang_arg("-Iatmel-start/hpl/core")
        .clang_arg("-Iatmel-start/hpl/dmac")
        .clang_arg("-Iatmel-start/hpl/gclk")
        .clang_arg("-Iatmel-start/hpl/pm")
        .clang_arg("-Iatmel-start/hpl/port")
        .clang_arg("-Iatmel-start/hpl/sysctrl")
        .clang_arg("-Iatmel-start/hri")
        .clang_arg("-Iatmel-start/samd21a/include")
        .header("atmel-start/atmel_start.h")
        .header("src/hal.h")
        .ctypes_prefix("cty")
        .use_core()
        .trust_clang_mangling(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
