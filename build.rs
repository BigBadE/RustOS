use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let kernel = env::var("CARGO_BIN_FILE_KERNEL_kernel").unwrap();
    let kernel = Path::new(&kernel);

    // create an UEFI disk image (optional)
    let uefi_path = out_dir.clone() + "/uefi.img";
    bootloader::UefiBoot::new(&kernel).create_disk_image(Path::new(&uefi_path)).unwrap();

    // create a BIOS disk image (optional)
    let bios_path = out_dir.clone() + "/bios.img";
    bootloader::BiosBoot::new(&kernel).create_disk_image(Path::new(&bios_path)).unwrap();

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path);
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path);
}