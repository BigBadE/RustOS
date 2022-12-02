use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let kernel = env::var("CARGO_BIN_FILE_KERNEL_kernel").unwrap();
    let kernel = Path::new(&kernel);

    let path = out_dir.clone() + "../../../../include";
    let path = Path::new(path.as_str());
    let output = out_dir.clone() + "included";
    let output = Path::new(output.as_str());

    let _ = fs::remove_file(output);

    read_into(path.clone().to_path_buf(), &mut File::create(output).unwrap());

    // create an UEFI disk image (optional)
    let uefi_path = out_dir.clone() + "/uefi.img";
    bootloader::UefiBoot::new_including(&kernel, output).create_disk_image(Path::new(&uefi_path)).unwrap();

    // create a BIOS disk image (optional)
    let bios_path = out_dir.clone() + "/bios.img";
    bootloader::BiosBoot::new_including(&kernel, output).create_disk_image(Path::new(&bios_path)).unwrap();

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path);
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path);
}

fn read_into(folder: PathBuf, output: &mut File) {
    let mut buffer = Vec::new();
    for file in fs::read_dir(folder.clone())
        .expect(format!("Error finding folder: {}", folder.to_str().unwrap()).as_str()) {
        let found = file.unwrap();
        if found.file_type().unwrap().is_dir() {
            read_into(found.path(), output);
        } else {
            buffer.clear();
            File::open(found.path()).unwrap().read_to_end(&mut buffer).expect("Failed to read file");
            output.write_all(buffer.as_slice()).expect("Failed to write to file");
        }
    }
}