

pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { v: [ x, y, z ] }
    }

    pub fn length(self) -> f64 {
        (self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;
    #[test]
    fn test_vector_length() {
        assert_eq!(
            Vec3::new(12.0, 5.0, -9.0).length(),
            250_f64.sqrt()
        )
    }
}
