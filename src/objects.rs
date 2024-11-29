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
}