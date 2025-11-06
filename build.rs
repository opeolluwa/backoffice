use std::fs;
use fs_extra::dir::{copy, CopyOptions};

fn main() {
    println!("cargo:rerun-if-changed=ui/.output/public");

    let dest = "./assets";
    if fs::metadata(dest).is_ok() {
        fs::remove_dir_all(dest).unwrap();
    }
    fs::create_dir_all(dest).unwrap();

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    copy("ui/.output/public/", dest, &options).unwrap();
}

