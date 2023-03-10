fn main() {
    #[cfg(feature = "gen_cbindings")]
    {
        use std::env;
        let root = env::var("CARGO_MANIFEST_DIR").unwrap();
        let _ = cbindgen::generate(&root)
            .map(|h| h.write_to_file("include/giga_segy.h"))
            .map_err(|error_message| println!("cargo:warning={}", error_message));

        // the following generates an environment variable `DEP__RUST_SEGY_GIGA_INCLUDE`
        // the double `_` is due to having `links = ""` set empty in `cargo.toml`
        println!("cargo:RUST_SEGY_GIGA_INCLUDE={}/include", root.as_str());
    }
}
