use libheif_rs::{RgbChroma, ColorSpace, HeifContext, Channel};

use crate::image::Image;

pub struct HeifLoader {
    pub filename: String,
    image: Image
}

impl HeifLoader {
    pub fn new(s: &String) -> HeifLoader {
        // Create a new loader for the given file
        return HeifLoader { filename: s.to_string(), image: Image::new(1, 1) };
    }
    pub fn load(&mut self) -> i32 {
        // Create the context
        let ctx = match HeifContext::read_from_file(self.filename.as_str()) {
            Ok(v) => v,
            Err(_error) => return 1
        };
        // Create the handle
        let handle = match ctx.primary_image_handle() {
            Ok(v) => v,
            Err(_error) => return 1
        };
        println!("\nImage has alpha channel: {}.\n", handle.has_alpha_channel());
        println!("CBPP: {}", handle.chroma_bits_per_pixel());
        // Create our internal image representation
        self.image = Image::new(handle.width() as i64, handle.height() as i64);
        // Decode the image
        let heifimage = match handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), false) {
            Ok(v) => v,
            Err(_error) => return 1
        };
        // Get the pixels
        let planes = heifimage.planes();
        let interleaved_plane = planes.interleaved.unwrap();
        // Initialize our internal image from the u8s
        println!("{:?}", interleaved_plane.bits_pre_pixel);
        self.image.load_from_rgb_interleaved_u8(interleaved_plane.data);
        return 0;
    }
    pub fn get_image(&self) -> Image {
        let mut i = Image::new(1, 1);
        self.image.copy_into(&mut i);
        return i;
    }
}