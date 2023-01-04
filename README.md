# giga-segy
A set of tools for reading and writing SEGY files conforming to the SEG Technical Standards Committee's [SEG-Y_r2.0 standard](https://seg.org/Portals/0/SEG/News%20and%20Resources/Technical%20Standards/seg_y_rev2_0-mar2017.pdf), written in the Rust programming language.

This workspace consists of three libraries:
* giga-segy-core
* giga-segy-in
* giga-segy-out
___
## giga-segy-core
This library provides the core structures and functionality that is used by `giga-segy-in` and `giga-segy-out`.

## giga-segy-in
This library provides the functionality that allows the reading of SEG-Y files. When a file is initially parsed, various headers and trace data becomes accessible. The library has some capacity to automatically detect endianness, coordinate and data format and text encoding. For cases where the file is written in an unusual manner (eg unusual byte indices for inline/crossline coordinates in trace headers), this library allows the manual specification of parameters when opening the file.

## giga-segy-out
This is a simple library that allows the writing of SEG-Y files, including the use of customised byte coordinate indices, coordinate format and trace data format. It does not accommodate the editing of SEG-Y files.

___
## License
* Apache Licencse, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0).
* MIT License (https://opensource.org/licenses/MIT)