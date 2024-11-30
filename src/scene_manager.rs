use std::fs::File;
use serde_json::{Result, Value};
use super::objects::{Sphere, Ray};
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
                sphere["color"][0].as_u64().unwrap() as u8,
                sphere["color"][1].as_u64().unwrap() as u8,
                sphere["color"][2].as_u64().unwrap() as u8,
            ];
            scene.spheres.push(Sphere::new(center, radius, color));
        }
        println!("{}", scene.spheres[0].center[0]);
        println!("{}", scene.spheres[0].center[1]);
        println!("{}", scene.spheres[0].center[2]);
        println!("{}", scene.spheres[0].radius);
        println!("{}", scene.spheres[0].color[0]);
        println!("{}", scene.spheres[0].color[1]);
        println!("{}", scene.spheres[0].color[2]);
        scene
    }
    pub fn trace(&self, ray: &Ray) -> [u8; 3] {
        let mut min_t = f32::INFINITY;
        let mut color = [0, 0, 0];
        for sphere in &self.spheres {
            //println!("speher");
            let t = sphere.intersection(ray);
            if t > 0.0 && t < min_t {
                min_t = t;
                color = sphere.color;
            }
        }
        color
    }
}