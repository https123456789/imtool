pub struct ImageType {
    heif: bool,
    jpeg: bool,
    png: bool,
    error: bool
}

impl ImageType {
    pub fn new(s: String) -> ImageType {
        let mut is_heif = false;
        let mut is_jpeg = false;
        let mut is_png = false;
        let mut is_error = false;
        if s.eq(&String::from("heif")) {
            is_heif = true;
        } else if s.eq(&String::from("jpeg")) {
            is_jpeg = true;
        } else if s.eq(&String::from("png")) {
            is_png = true;
        } else {
            is_error = true;
        }
        
        return ImageType { heif: is_heif, jpeg: is_jpeg, png: is_png, error: is_error };
    }
    pub fn to_string(&self) -> String {
        let mut rs = String::from("unknown");
        if self.heif {
            rs = String::from("heif");
        }
        if self.jpeg {
            rs = String::from("jpeg");
        }
        if self.png {
            rs = String::from("png");
        }
        return rs.to_string();
    }
}