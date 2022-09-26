use std::env;
use std::any::type_name;
use term_size;
use libheif_rs::{Channel, RgbChroma, ColorSpace, HeifContext, Result, ItemId};

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
    pub fn load(&self) -> i32 {
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
        // Create our internal image representation
        let mut image = Image::new(handle.width() as i64, handle.height() as i64);
        /*match env::var("IMTOOL_DEBUG") {
            Ok(_value) => {
                println!("\x1b[0m[DEBUG] Heif Loader: Image Size: ({:?}, {:?}).", handle.width(), handle.height())
            },
            Err(_error) => ()
        };*/
        // Get EXIF
        /*let mut meta_ids: Vec<ItemId> = vec![0; 1];
        let exif_block_ids = handle.metadata_block_ids("Exif", &mut meta_ids);
        let exif: Vec<u8> = match handle.metadata(meta_ids[0]) {
            Ok(v) => v,
            Err(_error) => return 1
        };*/
        // Decode the image
        let heifimage = match handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), false) {
            Ok(v) => v,
            Err(_error) => return 1
        };
        // Get the pixels
        let planes = heifimage.planes();
        let interleaved_plane = planes.interleaved.unwrap();
        // Initialize our internal image from the u8s
        image.load_from_u8(interleaved_plane.data);
        return 0;
    }
    pub fn get_image(&self) -> Image {
        let mut i = Image::new(1, 1);
        self.image.copy_into(&mut i);
        return i;
    }
}