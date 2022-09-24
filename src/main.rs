extern crate clap;
extern crate indicatif;
//extern crate bytes;

mod image_type;

//use std::fs::File;
//use std::io;
//use std::io::prelude::*;
use std::fs;

use clap::{Command, Arg};
//use indicatif::ProgressBar;
//use bytes::{BytesMut, BufMut};

use image_type::ImageType;

fn main() {
    // Setup args
    let arg_matches = Command::new("imtool")
        .author("Ben Landon")
        .version("0.1.0")
        .about("A command-line tool for working with images.")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .takes_value(false)
                .help("Print verbose information")
        )
        .arg(
            Arg::with_name("files")
                .multiple(true)
                .required(true)
                .index(1)
                .value_parser(clap::value_parser!(String))
        )
        .arg(
            Arg::new("to")
                .short('t')
                .long("to")
                .takes_value(true)
                .value_parser(clap::value_parser!(String))
                .help("Specifies the target image type to convert the image/s into.")
        )
        .get_matches();
    // Process input files
    let mut files: Vec<String> = Vec::new();
    let mut arg_files = arg_matches.get_many::<String>("files")
        .unwrap()
        .map(|s| s.as_str());
    loop {
        let afile = arg_files.next();
        if let None = afile {
            break;
        }
        files.push(
            afile.unwrap()
                .to_string()
        );
    }
    let mut target_type_string = String::from("###no-op###");
    if let Some(t) = arg_matches.get_one::<String>("to") {
        target_type_string = t.to_string();
    }
    for (pos, e) in files.iter().enumerate() {
        println!("File {}: {}", pos, e);
        process_file(&e, &String::from(&target_type_string));
    }
}

fn detect_image_type(filename: &String) -> ImageType {
    let fr = fs::read(filename);
    let mut file_type_string: String = String::from("error");
    if fr.is_ok() != true {
        println!("\x1b[31mError: file '{}' does not exist or could not be opened.\x1b[0m", filename);
        return ImageType::new(String::from("error"));
    }
    let file_data = fr.unwrap();
    // Detect the image type
    if file_data[0] == 0 && file_data[1] == 0 && file_data[2] == 0 {
        file_type_string = String::from("heif");
    } else if file_data[0] == 255 && file_data[1] == 216 {
        file_type_string = String::from("jpeg");
    } else if file_data[0] == 137 && file_data[1] == 80 && file_data[2] == 78 &&
                file_data[3] == 71 && file_data[4] == 13 && file_data[5] == 10 &&
                file_data[6] == 26 && file_data[7] == 10 {
        file_type_string = String::from("png");
    } else {}
    //println!("Size: {:?}, Type: {:?}", file_data.len(), file_type_string);
    return ImageType::new(file_type_string);
}

fn process_file(filename: &String, target_type_string: &String) -> i32 {
    let image_type = detect_image_type(filename);
    let mut target_type = ImageType::new(target_type_string.to_string());
    if target_type_string == "###no-op###" {
        target_type = ImageType::new(image_type.to_string());
    }
    println!("Type: {}, Target: {}", image_type.to_string(), target_type.to_string());
    return 0;
}