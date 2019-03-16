use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;

#[derive(Debug)]
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

impl Sub for Vec3{
    type Output = Vec3;
    fn sub(self, rhs:Vec3) -> Vec3{

        Vec3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,

        }
    }
}

impl Mul for Vec3{
    type Output = Vec3;
    fn mul(self, rhs:Vec3) -> Vec3{

        Vec3{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,

        }
    }
}

impl Div for Vec3{
    type Output = Vec3;
    fn div(self, rhs:Vec3) -> Vec3{

        Vec3{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,

        }
    }
}

impl Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Vec3{

        Vec3{
            x: - self.x ,
            y: - self.y,
            z: - self.z,

        }
    }
}


impl Mul<f32> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs:f32) -> Vec3{

        Vec3{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,

        }
    }
}

impl Div<f32 >for Vec3{
    type Output = Vec3;
    fn div(self, rhs:f32) -> Vec3{

        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,

        }
    }
}

impl Vec3{
    fn length(self) -> f32{
        ((self.x*self.x + self.y*self.y + self.z*self.z) as f32).sqrt()
    }

    fn length_squared(self) -> f32{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
}

impl Vec3{

    fn new(x:f32,y:f32, z:f32) -> Vec3{
        Vec3{
            x,y,z
        }
    }

    fn dot(self, new_vec:Vec3) -> f32{
        self.x * new_vec.x + self.y * new_vec.y + self.z * new_vec.z
    }

    fn cross(self, new_vec:Vec3) -> Vec3{
        Vec3{
            x: self.y*new_vec.z - self.z*new_vec.y,
            y: self.z*new_vec.x - self.x*new_vec.z,
            z: self.x*new_vec.y - self.y*new_vec.x,
        }
    }

    fn make_unit_vector(mut self) {
        let k = 1.0/((self.x*self.x + self.y*self.y + self.z*self.z) as f32).sqrt();
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }
}

