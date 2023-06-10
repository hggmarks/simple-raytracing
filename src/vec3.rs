

pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { v: [ x, y, z ] }
    }

    pub fn lenth(self) -> f64 {
        0.0 
    }
}
