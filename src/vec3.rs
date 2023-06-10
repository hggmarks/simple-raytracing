use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
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
    
    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }
}

use ops::Add;
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.v[0] + rhs.v[0],
            self.v[1] + rhs.v[1],
            self.v[2] + rhs.v[2]
        )
    }
}

use ops::Sub;
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.v[0] - rhs.v[0],
            self.v[1] - rhs.v[1],
            self.v[2] - rhs.v[2]
        )
    }
}


use ops::Mul;
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.v[0] * rhs,
            self.v[1] * rhs,
            self.v[2] * rhs
        )
    }
}

use ops::Div;
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let k: f64 = 1.0 / rhs;
        Vec3::new(
            self.v[0] * k,
            self.v[1] * k,
            self.v[2] * k,
        )
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

    #[test]
    fn test_add_vectors() {
        assert_eq!(
            Vec3::new(-4.5, 12.0, 70.0) + Vec3::new(4.0, 8.0, -1.0),
            Vec3::new(-0.5, 20.0, 69.0)
        )
    }

    #[test]
    fn test_sub_vectors() {
        assert_eq!(
            Vec3::new(70.0, 91.0, 100.0) - Vec3::new(1.0, 22.0, 31.0),
            Vec3::new(69.0, 69.0, 69.0)
        )
    }

    #[test]
    fn test_mul_vec_by_scalar() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 6.0) * 1.5,
            Vec3::new(1.5, 3.0, 9.0)
        )
    }

    #[test]
    fn test_div_vec_by_scalar() {
        assert_eq!(
            Vec3::new(1.5, 4.0, 12.0) / 2.0,
            Vec3::new(0.75, 2.0, 6.0)
        )
    }
}
