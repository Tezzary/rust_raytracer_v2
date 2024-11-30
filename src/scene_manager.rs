use std::fs::File;
use serde_json::{Result, Value};
use super::objects::{Sphere, Ray, Hit};
//use rand::Rng;
pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new(scene_name: String) -> Scene {
        let mut scene = Scene {
            spheres: Vec::new(),
        };
        let file = File::open(scene_name).expect("File not found");
        let data: Value = serde_json::from_reader(file).expect("Error while reading file");
        for sphere in data["spheres"].as_array().unwrap() {
            let center = [
                sphere["center"][0].as_f64().unwrap() as f32,
                sphere["center"][1].as_f64().unwrap() as f32,
                sphere["center"][2].as_f64().unwrap() as f32,
            ];
            let radius = sphere["radius"].as_f64().unwrap() as f32;
            let color = [
                sphere["color"][0].as_f64().unwrap() as f32,
                sphere["color"][1].as_f64().unwrap() as f32,
                sphere["color"][2].as_f64().unwrap() as f32,
            ];
            let light = sphere["light"].as_f64().unwrap() as f32;
            scene.spheres.push(Sphere::new(center, radius, color, light));
        }
        /* 
        println!("{}", scene.spheres[0].center[0]);
        println!("{}", scene.spheres[0].center[1]);
        println!("{}", scene.spheres[0].center[2]);
        println!("{}", scene.spheres[0].radius);
        println!("{}", scene.spheres[0].color[0]);
        println!("{}", scene.spheres[0].color[1]);
        println!("{}", scene.spheres[0].color[2]);
        */
        scene
    }
    pub fn trace(&self, ray: &mut Ray, bounces: usize, samples: usize) -> [u8; 3] {
        let mut colour_sum = [0.0, 0.0, 0.0];
        let initial_origin = ray.origin;
        let initial_direction = ray.direction;
        for _ in 0..samples {

            ray.origin = initial_origin;
            ray.direction = initial_direction;

            for _ in 0..bounces {
                let mut closest_hit = Hit::new(f32::INFINITY, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0);
                for sphere in &self.spheres {
                    //println!("speher");
                    let hit = sphere.intersection(ray);
                    if hit.t > 0.0 && hit.t < closest_hit.t {
                        closest_hit = hit;
                    }
                }
                if closest_hit.t == f32::INFINITY {
                    break;
                }
                ray.color[0] *= closest_hit.color[0] as f32;
                ray.color[1] *= closest_hit.color[1] as f32;
                ray.color[2] *= closest_hit.color[2] as f32;
                ray.origin = closest_hit.location;
                
                //not right
                ray.direction = closest_hit.normal;

                ray.light += closest_hit.light;
            }
            colour_sum[0] += ray.color[0] * ray.light;
            colour_sum[1] += ray.color[1] * ray.light;
            colour_sum[2] += ray.color[2] * ray.light;
        }
        [
            (colour_sum[0] * 255.0 / samples as f32) as u8,
            (colour_sum[1] * 255.0 / samples as f32) as u8,
            (colour_sum[2] * 255.0 / samples as f32) as u8,
        ]
    }
}