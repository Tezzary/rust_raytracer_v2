
use core::f32;

use png;
mod scene_manager;
mod png_manager;
mod objects;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const FOV: f32 = 90.0 * f32::consts::PI / 180.0; // 90 degrees
fn main() {
    let mut image = png_manager::Image::new(WIDTH as u32, HEIGHT as u32);
    let scene = scene_manager::Scene::new("scenes/scene_1.json".to_string());

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let x_angle = ((x as f32 + 0.5) / WIDTH as f32 - 0.5) * FOV;
            let y_angle = ((y as f32 + 0.5) / HEIGHT as f32 - 0.5) * FOV * HEIGHT as f32 / WIDTH as f32;
            let dir_x = x_angle.cos();
            let dir_y = y_angle.sin();
            let length = (dir_x.powi(2) + dir_y.powi(2) + 1.0).sqrt();
            let ray = objects::Ray::new([0.0, 0.0, -100.0], [dir_x / length, dir_y / length, 1.0 / length]);
            let color = scene.trace(&ray);
            image.set_pixel(x as u32, y as u32, [color[0], color[1], color[2], 255]);
        }
    }
    image.save_image();
}

