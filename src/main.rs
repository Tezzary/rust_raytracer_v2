
use png;
mod png_manager;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
fn main() {
    let mut image = png_manager::Image::new(WIDTH as u32, HEIGHT as u32);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = (x as f32 / WIDTH as f32 * 255.0) as u8;
            let g = (y as f32 / HEIGHT as f32 * 255.0) as u8;
            let b = 0;
            let a = 255;
            image.set_pixel(x as u32, y as u32, [r, g, b, a]);
        }
    }
    image.save_image();
}

