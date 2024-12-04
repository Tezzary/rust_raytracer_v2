use super::objects;
//read from .obj file
pub fn extract_triangles(filename: &str, light: f32, color: [f32; 3]) -> Vec<objects::Triangle> {
    let mut triangles = vec![];
    let mut vertices = vec![];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        match words.next() {
            Some("v") => {
                let x: f32 = words.next().unwrap().parse().unwrap();
                let y: f32 = words.next().unwrap().parse().unwrap();
                let z: f32 = words.next().unwrap().parse().unwrap();
                vertices.push([x, y, z]);
            }
            Some("f") => {
                let mut face = [[0.0; 3]; 3];
                for i in 0..3 {
                    let index: usize = words.next().unwrap().parse().unwrap();
                    face[i] = vertices[index - 1];
                }
                triangles.push(objects::Triangle::new(face, color, light));
            }
            _ => {}
        }
    }
    triangles
}