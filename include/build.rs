use std::{env, fs};
use std::path::{Path};
use fs_extra::dir::CopyOptions;
use fs_extra::file;

const INCLUDING: [&str; 1] = ["testing"];

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let including = out_dir.clone() + "/../../../include";
    let including = Path::new(including.as_str());

    let _ = fs::remove_dir_all(including);

    fs::create_dir_all(including).expect("Error creating output");
    fs_extra::dir::copy(String::from(env::current_dir().unwrap().to_str().unwrap()) + "/including",
                        including, &CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 4086,
            copy_inside: true,
            content_only: true,
            depth: 0
        }).expect("Failed copy");
    let output = including.join("output").to_owned();
    fs::create_dir(output.clone()).unwrap();
    for string in INCLUDING {
        let target = env::var(format!("CARGO_BIN_FILE_{}_{}", string.to_uppercase(), string)).unwrap();
        file::copy(target, output.clone().join(string), &file::CopyOptions::new())
            .expect("Failed copy");
    }
}