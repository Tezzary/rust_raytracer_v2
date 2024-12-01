
use core::f32;
use std::thread::{self, Thread};
use std::sync::mpsc;

use png;
mod scene_manager;
mod png_manager;
mod objects;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const FOV: f32 = 90.0 * f32::consts::PI / 180.0; // 90 degrees
const SAMPLES_PER_PIXEL: usize = 2000;
const MAX_BOUNCES: usize = 200;
const SCENE_FILE: &str = "scenes/scene_0.json";

const THREAD_COUNT: usize = 20;
const THREAD_CHUNK_SIZE: usize = 20;


fn main() {
    if WIDTH % THREAD_CHUNK_SIZE != 0 || HEIGHT % THREAD_CHUNK_SIZE != 0 {
        panic!("Width and height must be divisible by thread chunk size");
    }

    let mut image = png_manager::Image::new(WIDTH as u32, HEIGHT as u32);
    let scene = scene_manager::Scene::new(SCENE_FILE.to_string());

    let mut threads = vec![];

    let (tx, rx) = mpsc::channel();

    let mut communications_senders = vec![];

    for i in 0..THREAD_COUNT {
        let tx = tx.clone();
        let (ttx, rrx) = mpsc::channel();
        communications_senders.push(ttx);
        let cloned_scene = scene.clone();
        threads.push(thread::spawn(move || {
            loop {
                let (x, y) = match rrx.recv() {
                    Ok((x, y)) => (x, y),
                    Err(_) => break,
                };
                if x == -1 && y == -1 {
                    break;
                }
                let x = x as usize;
                let y = y as usize;

                let init_x = x * THREAD_CHUNK_SIZE;
                let init_y = y * THREAD_CHUNK_SIZE;
                let mut colors: [[u8; 3]; THREAD_CHUNK_SIZE * THREAD_CHUNK_SIZE]   = [[0, 0, 0]; THREAD_CHUNK_SIZE * THREAD_CHUNK_SIZE];
                for y in init_y..init_y+THREAD_CHUNK_SIZE {
                    for x in init_x..init_x+THREAD_CHUNK_SIZE {
                        let x_angle = ((x as f32 + 0.5) / WIDTH as f32 - 0.5) * FOV;
                        let y_angle = ((y as f32 + 0.5) / HEIGHT as f32 - 0.5) * FOV * HEIGHT as f32 / WIDTH as f32;
                        let dir_x = x_angle.sin();
                        let dir_y = -y_angle.sin();
                        let length = (dir_x.powi(2) + dir_y.powi(2) + 1.0).sqrt();
                        let mut ray = objects::Ray::new([0.0, 0.0, -50.0], [dir_x / length, dir_y / length, 1.0 / length]);
                        let color = cloned_scene.trace(&mut ray, MAX_BOUNCES, SAMPLES_PER_PIXEL);
                        colors[(y - init_y) * THREAD_CHUNK_SIZE + (x - init_x)] = color;
                    }
                }
                tx.send((i, x, y, colors)).unwrap();
            }
        }));
    }
    let mut saved_chunks = 0;
    let mut working_threads = 0;
    for y in 0..HEIGHT/THREAD_CHUNK_SIZE {
        for x in 0..WIDTH/THREAD_CHUNK_SIZE {
            while working_threads < THREAD_COUNT {
                communications_senders[working_threads].send((x as i32, y as i32)).unwrap();
                working_threads += 1;
            }
            let (thread_wanted, x_arrived, y_arrived, colors) = rx.recv().unwrap();
            for offset_y in 0..THREAD_CHUNK_SIZE {
                for offset_x in 0..THREAD_CHUNK_SIZE {
                    let color = colors[offset_y * THREAD_CHUNK_SIZE + offset_x];
                    image.set_pixel((x_arrived * THREAD_CHUNK_SIZE + offset_x) as u32, (y_arrived * THREAD_CHUNK_SIZE + offset_y) as u32, [color[0], color[1], color[2], 255]);
                }
            }
            communications_senders[thread_wanted].send((x as i32, y as i32)).unwrap();
            image.update_filename(format!("subimages/output_{}.png", saved_chunks));
            image.save_image();
            saved_chunks += 1;
        }
    }
    let mut awaiting = threads.len();
    while awaiting > 0 {
        let (thread_wanted, x_arrived, y_arrived, colors) = rx.recv().unwrap();
        for offset_y in 0..THREAD_CHUNK_SIZE {
            for offset_x in 0..THREAD_CHUNK_SIZE {
                let color = colors[offset_y * THREAD_CHUNK_SIZE + offset_x];
                image.set_pixel((x_arrived * THREAD_CHUNK_SIZE + offset_x) as u32, (y_arrived * THREAD_CHUNK_SIZE + offset_y) as u32, [color[0], color[1], color[2], 255]);
            }
        }
        communications_senders[thread_wanted].send((-1, -1)).unwrap();
        
        image.update_filename(format!("subimages/output_{}.png", saved_chunks));
        image.save_image();
        saved_chunks += 1;

        awaiting -= 1;
    }
    for thread in threads {
        thread.join().unwrap();
    }
    /* 
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let x_angle = ((x as f32 + 0.5) / WIDTH as f32 - 0.5) * FOV;
            let y_angle = ((y as f32 + 0.5) / HEIGHT as f32 - 0.5) * FOV * HEIGHT as f32 / WIDTH as f32;
            let dir_x = x_angle.sin();
            let dir_y = -y_angle.sin();
            let length = (dir_x.powi(2) + dir_y.powi(2) + 1.0).sqrt();
            let mut ray = objects::Ray::new([0.0, 0.0, -50.0], [dir_x / length, dir_y / length, 1.0 / length]);
            let color = scene.trace(&mut ray, MAX_BOUNCES, SAMPLES_PER_PIXEL);
            image.set_pixel(x as u32, y as u32, [color[0], color[1], color[2], 255]);
        }
    }
    */
    image.update_filename(png_manager::create_unused_filename());
    image.save_image();
}

