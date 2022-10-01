#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        return Color { r, g, b, a};
    }
    pub fn to_string(&self) -> String {
        return String::from(format!("rgb({}, {}, {})", self.r, self.g, self.b));
    }
}

#[derive(Debug)]
pub struct Pixel {
    color: Color
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        return Pixel { color: Color::new(r, g, b, a) }
    }
    pub fn copy_into(&self, p: &mut Pixel) {
        p.color.r = self.color.r;
        p.color.g = self.color.g;
        p.color.b = self.color.b;
        p.color.a = self.color.a;
    }
}

pub struct Image {
    pub pixels: Vec<Pixel>,
    pub width: i64,
    pub height: i64
}

impl Image {
    pub fn new(width: i64, height: i64) -> Image {
        // Create a new image from the given width and height
        return Image { pixels: Vec::new(), width, height };
    }
    pub fn load_from_rgba_u8(&mut self, data: &[u8]) {
        // Loads the image from a u8 slice using the format
        // [ ..., r, g, b, a, r, g, b, a, ... ]
        
        // Clear our pixels
        self.pixels = Vec::new();
        
        // Iterate over each set of items
        let mut i = 0;
        while i < (data.len() - 4) {
            // Add a new pixel
            self.pixels.push(
                Pixel::new(data[i], data[i+1], data[i+2], data[i+3])
            );
            i += 4;
        }
        println!("\n\n{:?}\n\n", self.pixels[0].color);
    }
    pub fn load_from_rgb_u8(&mut self, data: &[u8]) {
        // Loads the image from a u8 slice using the format
        // [ ..., r, g, b, r, g, b, ... ]
        // Clear our pixels
        self.pixels = Vec::new();
        // Iterate over each set of items
        let mut i = 0;
        while i < (data.len() - 3) {
            // Add a new pixel
            self.pixels.push(
                Pixel::new(data[i], data[i+1], data[i+2], 255)
            );
            i += 3;
        }
        println!("\n\nColor: {:?}\n\n", self.pixels[0].color.to_string());
    }
    pub fn load_from_rgb_interleaved_u8(&mut self, data: &[u8]) {
        // Loads the image from a u8 slice using the format
        // [ rrr...rrr, ggg...ggg, bbb...bbb ]
        println!("\nLoading from interleaved u8.\n");
        let mut vp = Vec::new();

        // Create all of the pixels
        let mut i = 0;
        while i < (data.len() / 3) {
            vp.push(
                Pixel::new(0, 0, 0, 255)
            );
            i += 1;
        }
        println!("Pixels created.");

        // Load the actual data into the pixels
        let mut j = 0;
        while j < vp.len() {
            let mut p = Pixel::new(255, 0, 0, 255);
            p.color.r = data[j];
            p.color.g = data[vp.len() + j];
            p.color.b = data[(vp.len() * 2) + j];
            vp[j].copy_into(&mut p);
            self.pixels.push(p);
            j += 1;
        }
        println!("PData loaded");
        /*for (pos, pixel) in self.pixels.iter_mut().enumerate() {
            (*pixel).color.r = data[pos];
            (*pixel).color.g = data[*(&mut self.pixels.len()) + pos];
            (*pixel).color.b = data[(self.pixels.len() * 2) + pos];
        }*/
        println!("\n\nLen: {:?}, Color: {:?}\n\n", self.pixels.len(), self.pixels[0].color.to_string());
    }
    pub fn copy_into(&self, i: &mut Image) {
        // Copies all of the data memebers from this image into the provided image
        i.width = self.width;
        i.height = self.height;
        // Clear the pixels
        i.pixels = Vec::new();
        // Copy each pixel in self into i
        for (_pos, pixel) in self.pixels.iter().enumerate() {
            let mut p = Pixel::new(0, 0, 0, 1);
            pixel.copy_into(&mut p);
            i.pixels.push(p);
        }
    }
    pub fn get_raw_pixels(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        for p in &self.pixels {
            v.push(p.color.r);
            v.push(p.color.g);
            v.push(p.color.b);
            v.push(p.color.a);
        }
        return v.to_vec();
    }
    pub fn get_raw_pixels_a_stripped(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        for p in &self.pixels {
            v.push(p.color.r);
            v.push(p.color.g);
            v.push(p.color.b);
        }
        return v.to_vec();
    }
}