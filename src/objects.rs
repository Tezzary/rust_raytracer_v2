
pub struct Hit {
    pub t: f32,
    pub location: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
    pub light: f32,
    pub smoothness: f32,
}

impl Hit {
    pub fn new(t: f32, location: [f32; 3], normal: [f32; 3], color: [f32; 3], light: f32, smoothness: f32) -> Hit {
        Hit {
            t: t,
            location: location,
            normal: normal,
            color: color,
            light: light,
            smoothness: smoothness,
        }
    }

}

#[derive(Clone)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
    pub color: [f32; 3],
    pub light: f32,
    pub smoothness: f32,
}
impl Sphere {
    pub fn new(center: [f32; 3], radius: f32, color: [f32; 3], light: f32, smoothness: f32) -> Sphere {
        Sphere {
            center,
            radius,
            color,
            light,
            smoothness,
        }
    }
    pub fn intersection(&self, ray: &Ray) -> Hit {
        let a = ray.direction[0].powi(2) + ray.direction[1].powi(2) + ray.direction[2].powi(2);
        let b = 2.0 * (ray.direction[0] * (ray.origin[0] - self.center[0]) +
                       ray.direction[1] * (ray.origin[1] - self.center[1]) +
                       ray.direction[2] * (ray.origin[2] - self.center[2]));
        let c = (ray.origin[0] - self.center[0]).powi(2) +
                (ray.origin[1] - self.center[1]).powi(2) +
                (ray.origin[2] - self.center[2]).powi(2) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0.0);
        }
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < 0.00001 {
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0.0);
        }
        let mut location = [ray.origin[0] + ray.direction[0] * t,
                        ray.origin[1] + ray.direction[1] * t,
                        ray.origin[2] + ray.direction[2] * t];
        let location = [location[0] - self.center[0], location[1] - self.center[1], location[2] - self.center[2]];
        let length = (location[0].powi(2) + location[1].powi(2) + location[2].powi(2)).sqrt();
        let location = [location[0] / length * self.radius + self.center[0],
                        location[1] / length * self.radius + self.center[1],
                        location[2] / length * self.radius + self.center[2]];
        let normal = [(location[0] - self.center[0]) / self.radius,
                        (location[1] - self.center[1]) / self.radius,
                        (location[2] - self.center[2]) / self.radius];
        
        Hit::new(t, location, normal, self.color, self.light, self.smoothness)
    }
}

fn cross_product(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1],
     a[2] * b[0] - a[0] * b[2],
     a[0] * b[1] - a[1] * b[0]]
}
fn dot_product(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn subtract(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
#[derive(Clone)]

pub struct Triangle {
    pub vertices: [[f32; 3]; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
    pub light: f32,
    pub smoothness: f32,
}

impl Triangle {
    pub fn new(vertices: [[f32; 3]; 3], color: [f32; 3], light: f32, smoothness: f32) -> Triangle {
        let normal = [(vertices[1][1] - vertices[0][1]) * (vertices[2][2] - vertices[0][2]) - (vertices[1][2] - vertices[0][2]) * (vertices[2][1] - vertices[0][1]),
                        (vertices[1][2] - vertices[0][2]) * (vertices[2][0] - vertices[0][0]) - (vertices[1][0] - vertices[0][0]) * (vertices[2][2] - vertices[0][2]),
                        (vertices[1][0] - vertices[0][0]) * (vertices[2][1] - vertices[0][1]) - (vertices[1][1] - vertices[0][1]) * (vertices[2][0] - vertices[0][0])];
        let length = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
        let normal = [normal[0] / length, normal[1] / length, normal[2] / length];
        Triangle {
            vertices,
            normal,
            color,
            light,
            smoothness,
        }
    }
    pub fn intersection(&self, ray: &Ray) -> Hit {

        //converted math into rust
        //https://diegoinacio.github.io/computer-vision-notebooks-page/pages/ray-intersection_triangle.html 
        
        //let vertex 0 = a, vertex 1 = b, vertex 2 = c
        let ab = subtract(self.vertices[1], self.vertices[0]); //vertices[1] - vertices[0]
        let ca = subtract(self.vertices[0], self.vertices[2]); //vertices[0] - vertices[2]
        let bc = subtract(self.vertices[2], self.vertices[1]); //vertices[2] - vertices[1]
        
        let d = -dot_product(self.normal, self.vertices[0]);
        let normal_dot_dir = dot_product(self.normal, ray.direction);

        if normal_dot_dir.abs() < 0.00001 { //parrallel check
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0.0);
        }

        let normal_dot_origin = dot_product(self.normal, ray.origin);
        let t = -(normal_dot_origin + d) / normal_dot_dir;
        if t < 0.00001 {
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0.0);
        }
        let p = [ray.origin[0] + ray.direction[0] * t,
                 ray.origin[1] + ray.direction[1] * t,
                 ray.origin[2] + ray.direction[2] * t];
        let pa = dot_product(cross_product(ab, subtract(p, self.vertices[0])), self.normal);
        let pb = dot_product(cross_product(bc, subtract(p, self.vertices[1])), self.normal);
        let pc = dot_product(cross_product(ca, subtract(p, self.vertices[2])), self.normal);

        if pa >= 0.0 && pb >= 0.0 && pc >= 0.0 {
            if dot_product(ray.direction, self.normal) > 0.0 {
                return Hit::new(t, p, [-self.normal[0], -self.normal[1], -self.normal[2]], self.color, self.light, self.smoothness);
            }
            return Hit::new(t, p, self.normal, self.color, self.light, self.smoothness);
        }
        Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 0.0)
    }
}

/*
pub struct TriangleObject {
    pub triangles: Vec<Triangle>,
    pub color: [f32; 3],
    pub light: f32,

    pub align: [f32; 3],

    pub max: [f32; 3],
    pub min: [f32; 3],
    pub avg: [f32; 3],

}
impl TriangleObject {
    pub fn new(triangles: Vec<Triangle>, color: [f32; 3], light: f32) -> TriangleObject {
        let mut max = [0.0, 0.0, 0.0];
        let mut min = [0.0, 0.0, 0.0];
        
        for triangle in &triangles {
            for i in 0..3 {
                for j in 0..3 {
                    if triangle.vertices[i][j] > max[j] {
                        max[j] = triangle.vertices[i][j];
                    }
                    if triangle.vertices[i][j] < min[j] {
                        min[j] = triangle.vertices[i][j];
                    }
                    avg[j] += triangle.vertices[i][j];
                }
            }
        }
        let mut avg = [(max[0] + min[0]) / 2.0, (max[1] + min[1]) / 2.0, (max[2] + min[2]) / 2.0];
        PolygonalObject {
            triangles,
            color,
            light,
            max,
            min,
            avg,
        }
    }
}
*/
pub struct Ray {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
    pub color: [f32; 3],
    pub light: f32,
}
impl Ray {
    pub fn new(origin: [f32; 3], direction: [f32; 3]) -> Ray {
        Ray {
            origin,
            direction,
            color: [1.0, 1.0, 1.0],
            light: 0.0,
        }
    }
}