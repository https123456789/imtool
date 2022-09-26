extern crate clap;
extern crate indicatif;

mod image_type;
mod image;
mod loaders;

use std::fs;

//use std::{thread, time::Duration};

use clap::{Command, Arg};
use indicatif::{ProgressBar, ProgressStyle};

use image_type::ImageType;
use image::Image;
use loaders::{HeifLoader};

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
    // Create the progress bar
    let bar = ProgressBar::new(files.len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:50.white.on_black} {pos}/{len} {msg}"));
    for (_pos, e) in files.iter().enumerate() {
        bar.set_message(format!(
            "Converting {}...",
            &e
        ).to_string());
        process_file(&e, &String::from(&target_type_string));
        bar.inc(1);
    }
    bar.finish_with_message(format!(
        "✅ All done! Finished converting {} files.",
        files.len() as u64
    ));
}

fn get_image_data(filename: &String) -> Result<Vec<u8>, String> {
    let fr = fs::read(&filename);
    if fr.is_ok() != true {
        println!("\x1b[31mError: file '{}' does not exist or could not be opened.\x1b[0m", filename);
        return Err(format!("file '{}' does not exist or could not be opened.", filename).to_string());
    }
    Ok(fr.unwrap())
}
    
fn detect_image_type(filename: &String) -> ImageType {
    let mut file_type_string: String = String::from("error");
    let mut file_data: Vec<u8> = Vec::new();
    file_data = match get_image_data(filename) {
        Ok(v) => v,
        Err(_error) => panic!("Error detecting image type")
    };
    //file_data = ;
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
    return ImageType::new(file_type_string);
}

fn get_image(filename: &String, image_type: &ImageType, target_type: &ImageType) -> Image {
    // Heif
    if image_type.heif {
        // Create the loader
        let loader = HeifLoader::new(filename);
        // Load
        loader.load();
        // Return the image
        return loader.get_image();
    } else {
        // Clear the line
        /*if let Some((w, h)) = term_size::dimensions() {
            print!("\r");
            for i in 0..w {
                print!(" ");
            }
            print!("\r");
        }*/
        print!("\x1b[1A\x1b[2K\r\x1b[93m[Warning]: Unsupported file type: {:?}. Triggered by file {}.\x1b[0m\n\n", image_type.to_string(), filename.to_string());
        return Image::new(1, 1);
    }
}

fn process_file(filename: &String, target_type_string: &String) -> i32 {
    let image_type = detect_image_type(filename);
    let target_type = ImageType::new(target_type_string.to_string());
    if target_type_string == "###no-op###" {
        return 0;
    }
    // Get the image
    let image = get_image(filename, &image_type, &target_type);
    //println!("Type: {}, Target: {}", image_type.to_string(), target_type.to_string());
    //thread::sleep(Duration::from_millis(1000));
    return 0;
}