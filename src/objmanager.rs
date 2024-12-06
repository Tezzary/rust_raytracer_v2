use std::fs::File;
use std::io::{BufRead, BufReader};
use super::objects;
//read from .obj file
pub fn extract_triangles(filename: &str, translation: [f32; 3], scale: [f32; 3], light: f32, color: [f32; 3], smoothness: f32) -> Vec<objects::Triangle> {
    let mut triangles = vec![];
    let mut vertices = vec![];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut minmaxx = [0.0, 0.0];
    let mut minmaxy = [0.0, 0.0];
    let mut minmaxz = [0.0, 0.0];

    for line in reader.lines() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        match words.next() {
            Some("v") => {
                let x: f32 = words.next().unwrap().parse().unwrap();
                let y: f32 = words.next().unwrap().parse().unwrap();
                let z: f32 = words.next().unwrap().parse().unwrap();
                if x < minmaxx[0] {
                    minmaxx[0] = x;
                }
                if x > minmaxx[1] {
                    minmaxx[1] = x;
                }
                if y < minmaxy[0] {
                    minmaxy[0] = y;
                }
                if y > minmaxy[1] {
                    minmaxy[1] = y;
                }
                if z < minmaxz[0] {
                    minmaxz[0] = z;
                }
                if z > minmaxz[1] {
                    minmaxz[1] = z;
                }
                vertices.push([x, y, z]);
            }
            Some("f") => {
                let mut face = [[0.0; 3]; 3];
                for i in 0..3 {
                    //println!("{}", words.next().unwrap().split("/").collect::<Vec<&str>>()[0].parse().unwrap());
                    let index: usize = words.next().unwrap().split("/").collect::<Vec<&str>>()[0].parse().unwrap();
                    face[i] = vertices[index - 1];
                }
                triangles.push(objects::Triangle::new(face, color, light, smoothness));
            }
            _ => {}
        }
    }
    let mid_point = [(minmaxx[0] + minmaxx[1]) / 2.0, (minmaxy[0] + minmaxy[1]) / 2.0, (minmaxz[0] + minmaxz[1]) / 2.0];
    for triangle in &mut triangles {
        for i in 0..3 {
            for j in 0..3 {
                triangle.vertices[i][j] = (triangle.vertices[i][j] - mid_point[j]) * scale[j] + translation[j];
            }
        }
    }

    /*
    for triangle in &triangles {
        println!("{:?}", triangle.vertices);
    }
    */

    triangles
}