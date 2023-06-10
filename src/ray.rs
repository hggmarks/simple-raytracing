use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn ray(o: Vec3, d: Vec3) -> Ray {
        Ray { origin: (o), direction: (d) }
    }

    pub fn origin(self) -> Vec3 {
        self.origin 
    }
 
    pub fn direction(self) -> Vec3 {
        self.direction 
    }
    
    pub fn point_at_parameter(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

}
