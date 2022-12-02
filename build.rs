use std::{env, fs, io};
use std::fs::File;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let kernel = env::var("CARGO_BIN_FILE_KERNEL_kernel").unwrap();
    let kernel = Path::new(&kernel);

    let path = Path::new((out_dir.clone() + "../../include").as_str());
    let mut file = File::create(path).unwrap();

    read_into(PathBuf::from(out_dir + "../../include"), &mut file);

    // create an UEFI disk image (optional)
    let uefi_path = out_dir.clone() + "/uefi.img";
    bootloader::UefiBoot::new_including(&kernel, path).create_disk_image(Path::new(&uefi_path)).unwrap();

    // create a BIOS disk image (optional)
    let bios_path = out_dir.clone() + "/bios.img";
    bootloader::BiosBoot::new_including(&kernel, path).create_disk_image(Path::new(&bios_path)).unwrap();

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path);
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path);
}

fn read_into(folder: PathBuf, output: &mut File) {
    for file in fs::read_dir(folder).unwrap() {
        let found = file.unwrap();
        if found.file_type().unwrap().is_dir() {
            read_into(file.unwrap().path(), output);
        } else {
            io::copy(&mut File::create(found.path()).unwrap(), output).unwrap();
        }
    }
}