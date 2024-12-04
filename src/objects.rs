
pub struct Hit {
    pub t: f32,
    pub location: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
    pub light: f32,
}

impl Hit {
    pub fn new(t: f32, location: [f32; 3], normal: [f32; 3], color: [f32; 3], light: f32) -> Hit {
        Hit {
            t: t,
            location: location,
            normal: normal,
            color: color,
            light: light,
        }
    }

}

#[derive(Clone)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
    pub color: [f32; 3],
    pub light: f32,
}
impl Sphere {
    pub fn new(center: [f32; 3], radius: f32, color: [f32; 3], light: f32) -> Sphere {
        Sphere {
            center,
            radius,
            color,
            light,
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
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0);
        }
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < 0.0 {
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0);
        }
        let mut location = [ray.origin[0] + ray.direction[0] * t,
                        ray.origin[1] + ray.direction[1] * t,
                        ray.origin[2] + ray.direction[2] * t];
        let location = [location[0] - self.center[0], location[1] - self.center[1], location[2] - self.center[2]];
        let length = (location[0].powi(2) + location[1].powi(2) + location[2].powi(2)).sqrt();
        let location = [location[0] / length * self.radius * 1.01 + self.center[0],
                        location[1] / length * self.radius * 1.00001 + self.center[1],
                        location[2] / length * self.radius * 1.00001 + self.center[2]];
        let normal = [(location[0] - self.center[0]) / self.radius,
                        (location[1] - self.center[1]) / self.radius,
                        (location[2] - self.center[2]) / self.radius];
        
        Hit::new(t, location, normal, self.color, self.light)
    }
}

pub struct Triangle {
    pub vertices: [[f32; 3]; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
    pub light: f32,
}
impl Triangle {
    pub fn new(vertices: [[f32; 3]; 3], color: [f32; 3], light: f32) -> Triangle {
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
        }
    }
    pub fn intersection(&self, ray: &Ray) -> Hit {
        let edge1 = [self.vertices[1][0] - self.vertices[0][0], self.vertices[1][1] - self.vertices[0][1], self.vertices[1][2] - self.vertices[0][2]];
        let edge2 = [self.vertices[2][0] - self.vertices[0][0], self.vertices[2][1] - self.vertices[0][1], self.vertices[2][2] - self.vertices[0][2]];
        let h = [ray.direction[1] * edge2[2] - ray.direction[2] * edge2[1],
                ray.direction[2] * edge2[0] - ray.direction[0] * edge2[2],
                ray.direction[0] * edge2[1] - ray.direction[1] * edge2[0]];
        let a = edge1[0] * h[0] + edge1[1] * h[1] + edge1[2] * h[2];
        if a > -0.00001 && a < 0.00001 {
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0);
        }
        let f = 1.0 / a;
        let s = [ray.origin[0] - self.vertices[0][0], ray.origin[1] - self.vertices[0][1], ray.origin[2] - self.vertices[0][2]];
        let u = f * (s[0] * h[0] + s[1] * h[1] + s[2] * h[2]);
        if u < 0.0 || u > 1.0 {
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0);
        }
        let q = [s[1] * edge1[2] - s[2] * edge1[1], s[2] * edge1[0] - s[0] * edge1[2], s[0] * edge1[1] - s[1] * edge1[0]];
        let v = f * (ray.direction[0] * q[0] + ray.direction[1] * q[1] + ray.direction[2] * q[2]);
        if v < 0.0 || u + v > 1.0 {
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0);
        }
        let t = f * (edge2[0] * q[0] + edge2[1] * q[1] + edge2[2] * q[2]);
        if t < 0.0 {
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0);
        }
        let location = [ray.origin[0] + ray.direction[0] * t,
                        ray.origin[1] + ray.direction[1] * t,
                        ray.origin[2] + ray.direction[2] * t];
        Hit::new(t, location, self.normal, self.color, self.light)
    }

pub struct PolygonalObject {
    pub triangles: Vec<Triangle>,
    pub color: [f32; 3],
    pub light: f32,

    pub align: [f32; 3],

    pub max: [f32; 3],
    pub min: [f32; 3],
    pub avg: [f32; 3],

}
impl PolygonalObject {
    pub fn new(triangles: Vec<Triangle>, color: [f32; 3], light: f32) -> PolygonalObject {
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
}
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