
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
        if t < 0.0001 {
            return Hit::new(-1.0, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0);
        }
        let location = [ray.origin[0] + ray.direction[0] * t,
                        ray.origin[1] + ray.direction[1] * t,
                        ray.origin[2] + ray.direction[2] * t];
        let normal = [(location[0] - self.center[0]) / self.radius,
                        (location[1] - self.center[1]) / self.radius,
                        (location[2] - self.center[2]) / self.radius];
        
        Hit::new(t, location, normal, self.color, self.light)
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