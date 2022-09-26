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
}

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
    pixels: Vec<Pixel>,
    width: i64,
    height: i64
}

impl Image {
    pub fn new(width: i64, height: i64) -> Image {
        // Create a new image from the given width and height
        return Image { pixels: Vec::new(), width, height };
    }
    pub fn load_from_u8(&mut self, data: &[u8]) {
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
    }
    pub fn copy_into<'a>(&'a self, i: &'a mut Image) {
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
}