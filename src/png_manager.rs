use std::fs::File;
use png;

pub fn create_unused_filename() -> String {
    let mut i = 0;
    loop {
        let filename = format!("images/output_{}.png", i);
        if !std::path::Path::new(&filename).exists() {
            return filename;
        }
        i += 1;
    }
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub filename: String,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
            filename: create_unused_filename(),
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
        let index = (x + y * self.width) as usize;
        self.data[index * 4] = color[0];
        self.data[index * 4 + 1] = color[1];
        self.data[index * 4 + 2] = color[2];
        self.data[index * 4 + 3] = color[3];
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let index = (x + y * self.width) as usize;
        [
            self.data[index * 4],
            self.data[index * 4 + 1],
            self.data[index * 4 + 2],
            self.data[index * 4 + 3],
        ]
    }
    pub fn update_filename(&mut self, filename: String) {
        self.filename = filename;
    }
    pub fn save_image(&self) {
        let file = File::create(&self.filename).unwrap();
        let ref mut w = std::io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.data).unwrap();
    }
}