use crate::image::Image;
use jpeg_encoder::{Encoder, ColorType};
use std::fs;

pub struct JpegExporter {
    
}

impl JpegExporter {
    pub fn new() -> JpegExporter {
        return JpegExporter {  };
    }
    pub fn export(&self, s: String, i: &Image) -> i32 {
        // Export the image to a JPEG file
        // Remove the file
        let rmr = fs::remove_file(s.as_str());
        if !rmr.is_ok() {
            println!("\nDEL\n");
        }
        // Create the encoder
        let e = Encoder::new_file(s.to_string(), 100);
        if !e.is_ok() {
            return -1;
        }
        let encoder = e.unwrap();
        // Get pixel data
        let px = (*i).get_raw_pixels_a_stripped();
        let pxs = &px[..];
        // Encode pixel data to jpeg data
        let er = encoder.encode(&pxs, (*i).width as u16, (*i).height as u16, ColorType::Rgb);
        println!("\n{:?}\n", er);
        if !er.is_ok() {
            return -1;
        }
        return 0;
    }
}