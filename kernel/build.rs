use std::env;
use std::fs;
use std::path::Path;

fn dir_walk(dir:&Path) -> () {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            dir_walk(&path);
        } else {
            let path_str = path.to_str().unwrap();
            if path_str.ends_with("asm") {
                let file_str = path.file_name().unwrap().to_str().unwrap();
                nasm_rs::compile_library_args(file_str, &[path_str], &["-f elf64", "-Ox"]).unwrap();
                println!("cargo:rerun-if-changed={}", path_str);
                println!("cargo:rustc-link-lib=static={}", file_str);
            }
        }
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    dir_walk(Path::new("src"));
}
