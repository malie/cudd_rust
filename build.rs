use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    println!("building libcudd.so");
    let st = Command::new("/bin/sh")
        .args(&["build-libcudd.sh"])
        .status().unwrap();
    if !st.success() {
        panic!("couldn't build libcudd, exit code: {}", st)}

    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=cudd")}
