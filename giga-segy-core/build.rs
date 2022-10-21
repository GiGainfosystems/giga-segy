extern crate cbindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let _ = cbindgen::generate(&root)
        .map(|h| h.write_to_file("include/giga_segy.h"))
        .map_err(|error_message| println!("cargo:warning={}", error_message));

    // Recently generation of the variabe `DEP__RUST_SEGY_GIGA_INCLUDE` has not been reliable,
    // therefore we copy the header directly to the build path (`target/debug` or `target/release`).
    let mut target = match env::var("OUT_DIR") {
        Ok(o) => PathBuf::from(o),
        Err(e) => panic!("cargo:warning=Error when copying \"giga_segy.h\": {}", e),
    };
    target.pop();
    target.pop();
    target.pop();

    let src = format!("{:?}/include/giga_segy.h", root.as_str());
    let src = src.replace('\"', "");
    let dest = format!("{:?}/giga_segy.h", target);
    let dest = dest.replace('\"', "");
    if let Err(e) = std::fs::copy(src, dest) {
        panic!("cargo:warning=Error when copying \"giga_segy.h\": {}", e);
    }
}
