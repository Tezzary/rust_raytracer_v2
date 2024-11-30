pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
    pub color: [u8; 3],
}
impl Sphere {
    pub fn new(center: [f32; 3], radius: f32, color: [u8; 3]) -> Sphere {
        Sphere {
            center,
            radius,
            color,
        }
    }
    pub fn intersection(&self, ray: &Ray) -> f32 {
        let a = ray.direction[0].powi(2) + ray.direction[1].powi(2) + ray.direction[2].powi(2);
        let b = 2.0 * (ray.direction[0] * (ray.origin[0] - self.center[0]) +
                       ray.direction[1] * (ray.origin[1] - self.center[1]) +
                       ray.direction[2] * (ray.origin[2] - self.center[2]));
        let c = ray.origin[0].powi(2) + ray.origin[1].powi(2) + ray.origin[2].powi(2) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return -1.0;
        }
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < 0.0 {
            return -1.0;
        }
        t
    }
}

pub struct Ray {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
    pub color: [u8; 3],
}
impl Ray {
    pub fn new(origin: [f32; 3], direction: [f32; 3]) -> Ray {
        Ray {
            origin,
            direction,
            color: [0, 0, 0],
        }
    }
}