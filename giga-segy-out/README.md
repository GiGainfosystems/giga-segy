# giga-segy-out
A set of tools for reading and writing SEGY files conforming to the SEG Technical Standards Committee's [SEG-Y_r2.0 standard](https://seg.org/Portals/0/SEG/News%20and%20Resources/Technical%20Standards/seg_y_rev2_0-mar2017.pdf), written in the Rust programming language.

`giga-segy-out` is part of the `giga-segy` library workspace, which is a tool for working with data in the SEG-Y format. The `giga-segy-out` library provides functionality for writing SEG-Y files of arbitrary size with a variety of options.

NB: It might be possible to edit SEG-Y files by using `giga-segy-in` and `giga-segy-out`, but this is not the intended use.

The library is quite lightweight, and uses a small number of dependencies. NB: Functionality for the production of C bindings for header structures requires the direct use of `giga-segy-core`.
___
## Getting started
Using the basic functionality of `giga-segy` is as simple as adding the dependencies to the `[dependencies]` section of the Cargo.toml of your project. Usually you only need `giga-segy-in` or `giga-segy-out` as they re-export all the necessities. However, for the generation of C bindings, you will need `giga-segy-core`.

```toml
[dependencies]
# I am using `giga-segy-out` for my writer.
giga-segy-out = "0.3.1"
# I only need core as a dependency because I want C bindings for the headers.
giga-segy-core = { version = "0.3.1", features = ["gen_cbindings"]}
```

Here is an example of a super simple SEG-Y parser that uses `giga-segy`.
```Rust
use giga_segy_out::SegyFile;
use giga_segy_core::{BinHeader, SegySettings, TraceHeader};
use giga_segy_core::enums::*;
use giga_segy_out::create_headers::{CreateBinHeader, CreateTraceHeader};
    
let dir = std::path::PathBuf::from("/keep/my/segy/here");
let path = dir.path().join("my-first-segy.sgy");

// Create a pretty much empty binary header. Only the byte indices are set.
// Everything else is `0` or something to the effect. 
let mut bin_header = BinHeader::default();
// We will attempt to convert all data to this format when writing.
bin_header.sample_format_code = SampleFormatCode::Float32;
// The number of samples in either the binary or trace header must equal data vector length.
bin_header.no_samples = 50;

// Here we create the file and write the tape label, binary header and text header.
let mut file = SegyFile::<SegySettings>::create_file(
    path,
    Default::default(),
    // This is just a fake text header. NB: Text header must be 3200 bytes long.
    std::iter::repeat('x').take(3200).collect::<String>(),
    bin_header,
    None,
).unwrap();

// Now we can add the data.
for i in 0..10 {
    // First we must create the trace header.
    let trace_header = TraceHeader::new_2d(1, 1, 0);
    // Then we take our data... (NB: As an example here is some fake data).
    // (NB: To disable lossy writing (eg f64 as f32), use `add_trace_lossless`)
    let data = (i..(i+50)).map(|x| x as f64).collect::<Vec<f64>>();
    // Finally write the trace, header and data, to the file.
    file.add_trace(trace_header, None, data).unwrap();
}
```
___
### Flavour
The library was designed to work foremost for the GiGa infosystems codebase and thus has something of a "GiGa flavour" to it.
___
## License
* Apache Licencse, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0).
* MIT License (https://opensource.org/licenses/MIT)