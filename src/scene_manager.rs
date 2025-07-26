use std::fs::File;
use serde_json::{Result, Value};
use super::objects::{Sphere, Ray, Hit, Triangle};
use rand::prelude::*;

use super::objmanager;

#[derive(Clone)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub triangles: Vec<Triangle>,
}
//sd 1, mean 0
fn gaussian_random(rng: &mut ThreadRng) -> f32 {
    let mut sum = 0.0;
    for _ in 0..12 {
        sum += rng.gen::<f32>();
    }
    sum - 6.0
}
fn specular_reflection(ray: &mut Ray, closest_hit: &Hit) -> [f32; 3] {
    let dot = ray.direction[0] * closest_hit.normal[0] +
              ray.direction[1] * closest_hit.normal[1] +
              ray.direction[2] * closest_hit.normal[2];
    [
        ray.direction[0] - 2.0 * dot * closest_hit.normal[0],
        ray.direction[1] - 2.0 * dot * closest_hit.normal[1],
        ray.direction[2] - 2.0 * dot * closest_hit.normal[2],
    ]
}

impl Scene {
    pub fn new(scene_name: String) -> Scene {
        let mut scene = Scene {
            spheres: Vec::new(),
            triangles: Vec::new(),
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
            let smoothness = sphere["smoothness"].as_f64().unwrap() as f32;
            scene.spheres.push(Sphere::new(center, radius, color, light, smoothness));
        }
        for obj in data["objects"].as_array().unwrap() {
            let filename = obj["filename"].as_str().unwrap();
            let light = obj["light"].as_f64().unwrap() as f32;
            let color = [
                obj["color"][0].as_f64().unwrap() as f32,
                obj["color"][1].as_f64().unwrap() as f32,
                obj["color"][2].as_f64().unwrap() as f32,
            ];
            let translation = [
                obj["position"][0].as_f64().unwrap() as f32,
                obj["position"][1].as_f64().unwrap() as f32,
                obj["position"][2].as_f64().unwrap() as f32,
            ];
            let scale = [
                obj["scale"][0].as_f64().unwrap() as f32,
                obj["scale"][1].as_f64().unwrap() as f32,
                obj["scale"][2].as_f64().unwrap() as f32,
            ];
            let smoothness = obj["smoothness"].as_f64().unwrap() as f32;
            let triangles = objmanager::extract_triangles(filename, translation, scale, light, color, smoothness);
            for triangle in triangles {
                scene.triangles.push(triangle);
            }
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
    pub fn trace(&self, x:usize, y:usize, bounces: usize, samples: usize, antialiasing: bool, width: usize, height: usize, fov: f32) -> [u8; 3] {
        let mut rng = rand::thread_rng();
        let mut antialiasing_x: f32 = 0.0;
        let mut antialiasing_y: f32 = 0.0;

        if antialiasing {
            antialiasing_x = rng.gen::<f32>() - 0.5;
            antialiasing_y = rng.gen::<f32>() - 0.5;
        }

        let x_angle = ((x as f32 + antialiasing_x + 0.5) / width as f32 - 0.5) * fov;
        let y_angle = ((y as f32 + antialiasing_y + 0.5) / height as f32 - 0.5) * fov * height as f32 / width as f32;

        let dir_x = x_angle.sin();
        let dir_y = -y_angle.sin();
        
        let length = (dir_x.powi(2) + dir_y.powi(2) + 1.0).sqrt();
        let mut ray = Ray::new([0.0, 0.0, 0.0], [dir_x / length, dir_y / length, 1.0 / length]);

        let mut colour_sum = [0.0, 0.0, 0.0];
        let initial_origin = [ray.origin[0], ray.origin[1], ray.origin[2]];

        

        for _ in 0..samples {

            ray.origin = [initial_origin[0], initial_origin[1], initial_origin[2]];

            if antialiasing {
                antialiasing_x = rng.gen::<f32>() - 0.5;
                antialiasing_y = rng.gen::<f32>() - 0.5;
            }
    
            let x_angle = ((x as f32 + antialiasing_x + 0.5) / width as f32 - 0.5) * fov;
            let y_angle = ((y as f32 + antialiasing_y + 0.5) / height as f32 - 0.5) * fov * height as f32 / width as f32;
    
            let dir_x = x_angle.sin();
            let dir_y = -y_angle.sin();
            ray.direction = [dir_x / length, dir_y / length, 1.0 / length];
            
            ray.color = [1.0, 1.0, 1.0];
            let mut accumulated_light = [0.0, 0.0, 0.0];

            for _ in 0..bounces {
                let mut closest_hit = Hit::new(f32::INFINITY, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0.0);
                for sphere in &self.spheres {
                    //println!("speher");
                    let hit = sphere.intersection(&ray);
                    if hit.t != -1.0 && hit.t < closest_hit.t {
                        closest_hit = hit;
                    }
                }
                for triangle in &self.triangles {
                    //println!("triangle");
                    let hit = triangle.intersection(&ray);
                    if hit.t != -1.0 && hit.t < closest_hit.t {
                        closest_hit = hit;
                    }
                }

                if closest_hit.t == f32::INFINITY {
                    break;
                }
                let light_emitted = [closest_hit.color[0] * closest_hit.light, closest_hit.color[1] * closest_hit.light, closest_hit.color[2] * closest_hit.light];
                accumulated_light = [accumulated_light[0] + light_emitted[0] * ray.color[0], accumulated_light[1] + light_emitted[1] * ray.color[1], accumulated_light[2] + light_emitted[2] * ray.color[2]];

                ray.color[0] *= closest_hit.color[0] as f32;
                ray.color[1] *= closest_hit.color[1] as f32;
                ray.color[2] *= closest_hit.color[2] as f32;

                // if closest_hit.light > 0.0 {
                //     break;
                // }

                ray.origin = closest_hit.location;
                
                //not right
                //ray.direction = closest_hit.normal;

                //mirror reflection
                /* 
                let dot = ray.direction[0] * closest_hit.normal[0] +
                          ray.direction[1] * closest_hit.normal[1] +
                          ray.direction[2] * closest_hit.normal[2];
                ray.direction[0] -= 2.0 * dot * closest_hit.normal[0];
                ray.direction[1] -= 2.0 * dot * closest_hit.normal[1];
                ray.direction[2] -= 2.0 * dot * closest_hit.normal[2];
                */
                //random reflection
                let x = gaussian_random(&mut rng);
                let y = gaussian_random(&mut rng);
                let z = gaussian_random(&mut rng);
                let length = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
                let mut new_ray_direction = [x / length, y / length, z / length];
                if new_ray_direction[0] * closest_hit.normal[0] + new_ray_direction[1] * closest_hit.normal[1] + new_ray_direction[2] * closest_hit.normal[2] < 0.0 {
                    new_ray_direction[0] = -new_ray_direction[0];
                    new_ray_direction[1] = -new_ray_direction[1];
                    new_ray_direction[2] = -new_ray_direction[2];
                }
                if closest_hit.smoothness > 0.0 {
                    let specular_direction = specular_reflection(&mut ray, &closest_hit);
                    ray.direction = [
                        new_ray_direction[0] * (1.0 - closest_hit.smoothness) + specular_direction[0] * closest_hit.smoothness,
                        new_ray_direction[1] * (1.0 - closest_hit.smoothness) + specular_direction[1] * closest_hit.smoothness,
                        new_ray_direction[2] * (1.0 - closest_hit.smoothness) + specular_direction[2] * closest_hit.smoothness,
                    ];
                }
                else {
                    ray.direction = new_ray_direction;
                }
            }
            colour_sum[0] += accumulated_light[0];
            colour_sum[1] += accumulated_light[1];
            colour_sum[2] += accumulated_light[2];
        }
        [
            (colour_sum[0] * 255.0 / samples as f32) as u8,
            (colour_sum[1] * 255.0 / samples as f32) as u8,
            (colour_sum[2] * 255.0 / samples as f32) as u8,
        ]
    }
}