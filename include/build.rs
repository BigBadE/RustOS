use std::{env, fs, io};
use std::fs::File;

const INCLUDING: [&str; 1] = ["testing"];

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    fs::create_dir(out_dir.clone() + "../../include").expect("Error creating output");
    for string in INCLUDING {
        let target = env::var(format!("CARGO_BIN_FILE_{}_{}", string.to_uppercase(), string)).unwrap();
        io::copy(&mut File::create(target).unwrap(),
                 &mut File::create(out_dir.clone() + "/output/" + string).unwrap()).expect("Failed copy");
    }
}