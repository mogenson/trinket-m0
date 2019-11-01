use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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

    // compile the atmel-start sources as a static library
    cc::Build::new()
        .define("__SAMD21E18A__", None)
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
        .file("atmel-start/samd21a/gcc/gcc/startup_samd21.c")
        .file("atmel-start/samd21a/gcc/system_samd21.c")
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
        .compile("libhal.a");
}
