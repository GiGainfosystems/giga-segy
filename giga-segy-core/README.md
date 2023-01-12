# giga-segy-core
A set of tools for reading and writing SEGY files conforming to the SEG Technical Standards Committee's [SEG-Y_r2.0 standard](https://seg.org/Portals/0/SEG/News%20and%20Resources/Technical%20Standards/seg_y_rev2_0-mar2017.pdf), written in the Rust programming language.

`giga-segy-core` is part of the `giga-segy` library workspace, which is a tool for working with data in the SEG-Y format. The `giga-segy-core` library provides the core structures and functionality that is shared by `giga-segy-in` and `giga-segy-out`.

The library is quite lightweight, but provides options (feature flags) for allowing serialization/deserialization via `serde`/`serde_json` and production of C bindings via `cbindgen`.
___
## Getting started
Using the basic functionality of `giga-segy` is as simple as adding the dependencies to the `[dependencies]` section of the Cargo.toml of your project. Usually you only need `giga-segy-in` or `giga-segy-out` as they re-export all the necessities. However, for the generation of C bindings, you will need `giga-segy-core`.

```toml
[dependencies]
# I am using `giga-segy-in` for my parser.
giga-segy-in = "0.3.1"
# I only need core as a dependency because I want C bindings for the headers.
giga-segy-core = { version = "0.3.1", features = ["gen_cbindings"]}
```

Here is an example of a super simple SEG-Y parser that uses `giga-segy`.
```Rust
use std::path::PathBuf;
use giga_segy_in::SegyFile;

let dir = PathBuf::from("/my/data/lives/here");
let full_path = dir.join("MyFavouriteSEGYDataset.sgy");

let file = SegyFile::open(name.to_str().unwrap(), Default::default()).unwrap();

// I want to get the text header and dump it to the terminal.
let text_header: &str = file.get_text_header();
println!("Text header: {:?}", text_header);

// Oops. SEG-Y headers look messy if we don't go line by line...
for line in file.get_text_header_lines() {
    println!("{}", line);
}

// Now to have a look at the binary header.
let bin_header = file.get_bin_header();
println!("Bin header: {}", bin_header);

// Get the data in the order of appearance of traces in the file.
// Of course there are more organised ways of doing this,
// but I just want to see the data...
for trace in file.traces_iter() {
    // First a quick peek at the trace header.
    println!("Trace header: {}", trace.get_header());
    // ..And then the data.
    // NB: trace data is not loaded to RAM until this is called.
    let data:Vec<f32> = file.get_trace_data_as_f32_from_trace(trace).unwrap();
    println!("Data: {:?}", data);
}
```
___
### Flavour
The library was designed to work foremost for the GiGa infosystems codebase and thus has something of a "GiGa flavour" to it.
___
## License
* Apache Licencse, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0).
* MIT License (https://opensource.org/licenses/MIT)