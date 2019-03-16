use std::ops::Add;

pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vec3{
    type Output = Vec3;
    fn add(self, rhs:Vec3) -> Vec3{

        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,

        }
    }
}
