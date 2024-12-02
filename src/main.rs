
use core::f32;
use std::thread::{self, Thread};
use std::sync::mpsc;
use std::io::Write;
use png;
use std::time;
mod scene_manager;
mod png_manager;
mod objects;

const LOGGING: bool = false;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;
const FOV: f32 = 90.0 * f32::consts::PI / 180.0; // 90 degrees
const SAMPLES_PER_PIXEL: usize = 50000;
const MAX_BOUNCES: usize = 20;
const ANTI_ALIASING: bool = true;
const SCENE_FILE: &str = "scenes/scene_0_wallless.json";

const THREAD_COUNT: usize = 24;
const THREAD_CHUNK_SIZE: usize = 20;

fn status_print(saved_chunks: usize, total_chunks: usize, elapsed_time: u64) {
    print!("Saved chunks: {}/{} Time Elapsed: {}s/{}s\r", saved_chunks, total_chunks, elapsed_time, elapsed_time * total_chunks as u64 / saved_chunks as u64);
    std::io::stdout().flush().unwrap();
}
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
            let mut rng = rand::thread_rng();
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
                        let color = cloned_scene.trace(x, y, MAX_BOUNCES, SAMPLES_PER_PIXEL, ANTI_ALIASING, WIDTH, HEIGHT, FOV);
                        colors[(y - init_y) * THREAD_CHUNK_SIZE + (x - init_x)] = color;
                    }
                }
                tx.send((i, x, y, colors)).unwrap();
            }
        }));
    }
    let mut saved_chunks = 0;
    let mut working_threads = 0;

    let start_time = time::Instant::now();
    for mut y in 0..HEIGHT/THREAD_CHUNK_SIZE {
        for x in 0..WIDTH/THREAD_CHUNK_SIZE {
            //very interesting render pattern when uncommented
            //y = HEIGHT/THREAD_CHUNK_SIZE - y - 1;

            if working_threads < THREAD_COUNT {
                communications_senders[working_threads].send((x as i32, y as i32)).unwrap();
                working_threads += 1;
                continue;
            }
            let (thread_wanted, x_arrived, y_arrived, colors) = rx.recv().unwrap();
            for offset_y in 0..THREAD_CHUNK_SIZE {
                for offset_x in 0..THREAD_CHUNK_SIZE {
                    let color = colors[offset_y * THREAD_CHUNK_SIZE + offset_x];
                    image.set_pixel((x_arrived * THREAD_CHUNK_SIZE + offset_x) as u32, (y_arrived * THREAD_CHUNK_SIZE + offset_y) as u32, [color[0], color[1], color[2], 255]);
                }
            }
            communications_senders[thread_wanted].send((x as i32, y as i32)).unwrap();

            if LOGGING {
                image.update_filename(format!("subimages/output_{}.png", saved_chunks));
                image.save_image();
            }

            saved_chunks += 1;
            status_print(saved_chunks, WIDTH/THREAD_CHUNK_SIZE * HEIGHT/THREAD_CHUNK_SIZE, start_time.elapsed().as_secs());
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

        if LOGGING {
            image.update_filename(format!("subimages/output_{}.png", saved_chunks));
            image.save_image();
        }
        saved_chunks += 1;
        status_print(saved_chunks, WIDTH/THREAD_CHUNK_SIZE * HEIGHT/THREAD_CHUNK_SIZE, start_time.elapsed().as_secs());

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
    println!("\nTotal time elapsed: {}s", start_time.elapsed().as_secs());
}

