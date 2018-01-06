[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

# barcode_gen

**barcode_gen** is a command-line utility to generate barcodes as PNG, GIF or SVG.

## Installation

Pre-compiled binaries exist for Linux x86 (64-bit) and Windows 7 (64-bit):
You'll find them under https://github.com/revilo/barcode_gen/releases/tag/v1.0-beta

At the moment there is no setup procedure available. Just copy the corresponding binary and run it.

## Usage

```
barcode_gen [OPTIONS] <TEXT-TO-ENCODE>
```

### Flags

| -h, --help               | Prints help information          |
| -V, --version            | Prints version information       |

### Options

| -f, --file <FILENAME>    | Specifies the file to write to (if not given, STDOUT is used)          | 
| --height <PIXELS>        | Specifies the height of the output image in pixels [default: 80]       |
| -o, --output <OUTPUT_FORMAT>    | Specifies which output format to use [default: PNG]             |
| --savespace <NUM_NARROW_BARS>   | Specifies that an empty 'save space' on the left and the right of the barcode should be added [default: 0]     |
| -t, --type <BARCODE_TYPE>       | Specifies which barcode type to generate [default: EAN13]       |
| --xdim <DIM>             | Specifies the width of the 'narrow' bars in pixels [default: 1]        |

### Arguments

| <TEXT-TO-ENCODE>   | Specifies the text to encode         | 

## Supported barcode types

Supported barcode types are:
EAN13, EAN8, CODE39, CODE93, CODE11, CODE128, CODABAR, ITF (2 of 5 Interleaved), STF (2 of 5 Standard).

## Supported output formats

Supported output formats are:
PNG, GIF, SVG

## Backend usage

This software uses the Rust library **Barcoders** as a backend.
Please see https://github.com/buntine/barcoders for further information.

## License

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
