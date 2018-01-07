[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache licensed](https://img.shields.io/badge/license-Apache2.0-green.svg)](./LICENSE-APACHE)

# barcode_gen

**barcode_gen** is a command-line utility to generate barcodes as PNG, GIF or SVG.

This software uses the Rust library _Barcoders_ as a backend.
Please see https://github.com/buntine/barcoders for further information.

## Installation

At the moment there is no setup procedure available. Just copy the corresponding binary and run it.

| Binary      | Version              | Description                      |
|-------------|----------------------|----------------------------------|
| [barcode_gen](https://github.com/revilo/barcode_gen/releases/download/v1.0-beta/barcode_gen)   | v1.0.0-beta | Binary for Linux x86-64  |
| [barcode_gen.exe](https://github.com/revilo/barcode_gen/releases/download/v1.0-beta/barcode_gen.exe)   | v1.0.0-beta | Binary for Windows 7 x86 (64 bit)  |

## Usage

```
barcode_gen [OPTIONS] <TEXT-TO-ENCODE>
```

| Argument                 | Description                      |
|--------------------------|----------------------------------|
| `-h, --help`             | Prints help information          |
| `-V, --version`          | Prints version information       |
| `-f, --file <NAME>`      | Specifies the file to write to (if not given, STDOUT is used)          | 
| `--height <PIXELS>`      | Specifies the height of the output image in pixels [default: 80]       |
| `-o, --output <FORMAT>`  | Specifies which output format to use [default: PNG]             |
| `--savespace <SIZE>`     | Specifies the size of the empty 'save space' on left and right of the barcode given in number of narrow bars [default: 0]       |
| `-t, --type <BARCODE>`   | Specifies which barcode type to generate [default: EAN13]       |
| `--xdim <DIM>`           | Specifies the width of the 'narrow' bars in pixels [default: 1]        |
| `<TEXT-TO-ENCODE>`       | Specifies the text to encode         | 

## Supported barcode types

Supported barcode types are:
`EAN13`, `EAN8`, `CODE39`, `CODE93`, `CODE11`, `CODE128`, `CODABAR`, `ITF` (2 of 5 Interleaved), `STF` (2 of 5 Standard).

## Supported output formats

Supported output formats are:
`PNG`, `GIF`, `SVG`

## Examples

Example: EAN13, PNG file
```
./barcode_gen 0000123456789 --file EAN13_0000123456789.png
```

Example: EAN8, save space, narrow bars witdh of 3 pixels, PNG file
```
./barcode_gen 12345678 --type EAN8 --file EAN8_12345678.png --savespace 10 --xdim 3
```

Example: CODE128, save space, narrow bars witdh of 3 pixels, PNG file
```
./barcode_gen "ÀBARCODE" --type CODE128 --file CODE128_BARCODE.png --savespace 10 --xdim 3
```
(À indicates a switch to codepage A) 

## License

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
