extern crate barcoders;
extern crate clap;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufWriter;
use clap::{Arg, App};

static BARCODE_TYPES : &[&str] = &[
    "EAN13",
    "EAN8",
    "CODE39",
    "CODE93",
    "CODE11",
    "CODE128",
    "CODABAR",
    "ITF",
    "STF"
];

static OUTPUT_FORMATS : &[&str] = &[
    "PNG",
    "GIF",
    "SVG"
];

fn main() {
    let app = App::new("barcode_gen")
        .version("1.0")
        .author("Oliver Dick <oliver.dick@evilstorm.de>")
        .about("Command Line Barcode Generator")
        .arg(Arg::with_name("text")
             .value_name("TEXT_TO_ENCODE")
             .help("Specifies the text to encode")
             .required(true)
             .takes_value(false))
        .arg(Arg::with_name("type")
             .short("t")
             .long("type")
             .value_name("BARCODE_TYPE")
             .help("Specifies which barcode type to generate")
             .possible_values(BARCODE_TYPES)
             .default_value("EAN13")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("OUTPUT_FORMAT")
             .help("Specifies which output format should be used")
             .possible_values(OUTPUT_FORMATS)
             .default_value("PNG")
             .takes_value(true))
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .value_name("FILENAME")
             .help("Specifies the file to write to (if not given STDOUT is used)")
             .takes_value(true))
        .arg(Arg::with_name("height")
             .long("height")
             .value_name("PIXELS")
             .default_value("80")
             .help("Specifies the height of the output image in pixels")
             .takes_value(true))
        .arg(Arg::with_name("savespace")
             .long("savespace")
             .value_name("NUM_NARROW BARS")
             .default_value("0")
             .help("Specifies that an empty 'save space' on the left and the right should be added")
             .takes_value(true))
        .arg(Arg::with_name("xdim")
             .long("xdim")
             .value_name("DIM")
             .default_value("1")
             .help("Specifies the width of the 'narrow' bars in pixels")
             .takes_value(true))
        .after_help("This software utilizes the barcoders library (see https://github.com/buntine/barcoders).");

    let matches = app.get_matches();

    let text_to_encode = matches.value_of("text").unwrap();
    let height = matches.value_of("height").unwrap().parse::<u32>().expect("Argument 'height' is not a valid integer");
    let xdim = matches.value_of("xdim").unwrap().parse::<u32>().expect("Argument 'xdim' is not a valid integer");
    let savespace = matches.value_of("savespace").unwrap().parse::<u32>().expect("Argument 'savespace' is not a valid integer");

    // encode
    let encoded = match matches.value_of("type").unwrap() {
        "EAN13" => {
            use barcoders::sym::ean13::EAN13;
            match EAN13::new(text_to_encode) {
                Ok(barcode) => { barcode.encode() }
                Err(err) => { panic!("Error: {}", err) }
            }
        }
        "EAN8" => {
            use barcoders::sym::ean8::EAN8;
            match EAN8::new(text_to_encode) {
                Ok(barcode) => { barcode.encode() }
                Err(err) => { panic!("Error: {}", err) }
            }
        }
        "CODE39" => {
            use barcoders::sym::code39::Code39;
            match Code39::new(text_to_encode) {
                Ok(barcode) => { barcode.encode() }
                Err(err) => { panic!("Error: {}", err) }
            }
        }
        "CODE93" => {
            use barcoders::sym::code93::Code93;
            match Code93::new(text_to_encode) {
                Ok(barcode) => { barcode.encode() }
                Err(err) => { panic!("Error: {}", err) }
            }
        }
        "CODE11" => {
            use barcoders::sym::code11::Code11;
            match Code11::new(text_to_encode) {
                Ok(barcode) => { barcode.encode() }
                Err(err) => { panic!("Error: {}", err) }
            }
        }
        "CODE128" => {
            use barcoders::sym::code128::Code128;
            match Code128::new(text_to_encode) {
                Ok(barcode) => { barcode.encode() }
                Err(err) => { panic!("Error: {}", err) }
            }
        }
        "CODABAR" => {
            use barcoders::sym::codabar::Codabar;
            match Codabar::new(text_to_encode) {
                Ok(barcode) => { barcode.encode() }
                Err(err) => { panic!("Error: {}", err) }
            }
        }
        "ITF" => {
            use barcoders::sym::tf::TF;
            match TF::interleaved(text_to_encode) {
                Ok(barcode) => { barcode.encode() }
                Err(err) => { panic!("Error: {}", err) }
            }
        }
        "STF" => {
            use barcoders::sym::tf::TF;
            match TF::standard(text_to_encode) {
                Ok(barcode) => { barcode.encode() }
                Err(err) => { panic!("Error: {}", err) }
            }
        }
        _ => {
            panic!("Invalid barcode type provided")
        }
    };

    // if save space is specified, add it
    let encoded = if savespace > 0 {
        let v : Vec<u8> = std::iter::repeat(0).take(savespace as usize).collect();
        let mut result = v.clone();
        result.extend(encoded);
        result.extend(v);
        result
    } else {
        // leave it as 
        encoded
    };

    // generate output format
    let output = match matches.value_of("output").unwrap() {
        "PNG" => {
            use barcoders::generators::image::{Color, Rotation};
            use barcoders::generators::image::Image::PNG;
            let png = PNG {
                height,
                xdim,
                rotation: Rotation::Zero,
                foreground: Color::new([0, 0, 0, 255]),
                background: Color::new([255, 255, 255, 255])
            };
            png.generate(&encoded[..]).unwrap()
        }
        "GIF" => {
            use barcoders::generators::image::{Color, Rotation};
            use barcoders::generators::image::Image::GIF;
            let gif = GIF {
                height,
                xdim,
                rotation: Rotation::Zero,
                foreground: Color::new([0, 0, 0, 255]),
                background: Color::new([255, 255, 255, 255])
            };
            gif.generate(&encoded[..]).unwrap()
        }
        "SVG" => {
            use barcoders::generators::svg::Color;
            use barcoders::generators::svg::SVG;
            let svg = SVG {
                height,
                xdim,
                foreground: Color::new([0, 0, 0, 255]),
                background: Color::new([255, 255, 255, 255])
            };
            svg.generate(&encoded[..]).unwrap().as_bytes().to_vec()
        }
        _ => {
            panic!("Invalid output type provided")
        }
    };

    // write to file or STDOUT
    match matches.value_of("file") {
        Some(filename) => {
            let file = File::create(&Path::new(filename)).unwrap();
            let mut writer = BufWriter::new(file);
            writer.write(&output).unwrap();
        }
        None => {
            std::io::stdout().write(&output).unwrap();
        }
    }    
}
