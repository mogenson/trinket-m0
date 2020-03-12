extern crate bindgen;
extern crate cc;

fn main() {
    let defines = ["__SAMD21E18A__"];

    let flags = [
        "-c",
        "-ffunction-sections",
        "-ggdb3",
        "-mcpu=cortex-m0plus",
        "-mlong-calls",
        "-mthumb",
        "-nostartfiles",
        "-Os",
        "-std=gnu99",
        "-Wall",
        "-Wno-cast-function-type",
        "-Wno-unused-parameter",
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
        "atmel-start/hpl/usb",
        "atmel-start/hri",
        "atmel-start/samd21a/include",
        "atmel-start/usb",
        "atmel-start/usb/class/cdc",
        "atmel-start/usb/class/cdc/device",
        "atmel-start/usb/device",
    ];

    /* exclude main.c and driver_examples.c */
    let files = [
        "atmel-start/atmel_start.c",
        "atmel-start/driver_init.c",
        "atmel-start/hal/src/hal_atomic.c",
        "atmel-start/hal/src/hal_delay.c",
        "atmel-start/hal/src/hal_gpio.c",
        "atmel-start/hal/src/hal_init.c",
        "atmel-start/hal/src/hal_io.c",
        "atmel-start/hal/src/hal_sleep.c",
        "atmel-start/hal/src/hal_spi_m_sync.c",
        "atmel-start/hal/src/hal_usb_device.c",
        "atmel-start/hal/utils/src/utils_assert.c",
        "atmel-start/hal/utils/src/utils_event.c",
        "atmel-start/hal/utils/src/utils_list.c",
        "atmel-start/hal/utils/src/utils_syscalls.c",
        "atmel-start/hpl/core/hpl_core_m0plus_base.c",
        "atmel-start/hpl/core/hpl_init.c",
        "atmel-start/hpl/dmac/hpl_dmac.c",
        "atmel-start/hpl/gclk/hpl_gclk.c",
        "atmel-start/hpl/pm/hpl_pm.c",
        "atmel-start/hpl/sercom/hpl_sercom.c",
        "atmel-start/hpl/sysctrl/hpl_sysctrl.c",
        "atmel-start/hpl/usb/hpl_usb.c",
        "atmel-start/samd21a/gcc/gcc/startup_samd21.c",
        "atmel-start/samd21a/gcc/system_samd21.c",
        "atmel-start/usb/class/cdc/device/cdcdf_acm.c",
        "atmel-start/usb/device/usbdc.c",
        "atmel-start/usb_start.c",
        "atmel-start/usb/usb_protocol.c",
        "src/hal.c",
    ];

    let mut builder = cc::Build::new();
    builder.pic(false);
    builder.no_default_flags(true);
    builder.compiler("arm-none-eabi-gcc");
    builder.archiver("arm-none-eabi-ar"); // adds flags "crs" by default

    /* uncomment below and use link arg "-lhal" to manually link libhal.a */
    // builder.out_dir(".");
    // builder.cargo_metadata(false);

    let mut bindings = bindgen::Builder::default();
    bindings = bindings.clang_arg("--sysroot=/usr/arm-none-eabi");
    bindings = bindings.header("atmel-start/driver_init.h");
    bindings = bindings.header("atmel-start/usb_start.h");
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
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings");
}
