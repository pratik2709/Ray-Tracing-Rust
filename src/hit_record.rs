#[derive(Clone)]
pub struct hit_record{
    pub t:f32,
    pub p:Vec3,
    pub normal:Vec3,
    pub material: Box<Material>
}

impl hit_record{

    fn getP(&self) -> Vec3{
        Vec3{
            x: self.p.x,
            y: self.p.y,
            z: self.p.z,
        }
    }

    fn getNormal(&self) -> Vec3{
        Vec3{
            x: self.normal.x,
            y: self.normal.y,
            z: self.normal.z,
        }
    }

    fn new(t:f32,p:Vec3,normal:Vec3, material: Material) -> hit_record{
        hit_record{
            t,p,normal, material
        }
    }
}