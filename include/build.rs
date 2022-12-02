use std::{env, fs, io};
use std::fs::File;
use std::path::Path;

const INCLUDING: [&str; 1] = ["testing"];

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let including = out_dir.clone() + "../../include";
    let including = Path::new(including.as_str());

    fs::remove_dir_all(including).unwrap();
    fs::create_dir(including).expect("Error creating output");
    fs::copy("include/including", including).unwrap();
    let output = including.join("output").to_owned();
    fs::create_dir(output).unwrap();
    for string in INCLUDING {
        let target = env::var(format!("CARGO_BIN_FILE_{}_{}", string.to_uppercase(), string)).unwrap();
        io::copy(&mut File::create(target).unwrap(),
                 &mut File::create(output.join(string)).unwrap())
            .expect("Failed copy");
    }
}