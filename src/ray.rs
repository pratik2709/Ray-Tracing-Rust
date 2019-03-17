pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray{

    pub fn getOrigin(&self) -> Vec3{
        Vec3{
            x:self.origin.x,
            y:self.origin.y,
            z:self.origin.z,
        }
    }

    pub fn getDirection(&self) -> Vec3{
        Vec3{
            x:self.direction.x,
            y:self.direction.y,
            z:self.direction.z,
        }

    }

    pub fn new(origin: Vec3, direction: Vec3) -> Ray{
        Ray{
            origin, direction
        }
    }

    pub fn point_at_parameter(self, t:f32) -> Vec3{
        self.origin + (self.direction * t)
    }
}
