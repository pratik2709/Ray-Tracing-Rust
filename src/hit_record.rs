#[derive(Clone)]
pub struct hit_record{
    pub t:f32,
    pub p:Vec3,
    pub normal:Vec3
}

impl hit_record{
    fn new(t:f32,p:Vec3,normal:Vec3) -> hit_record{
        hit_record{
            t,p,normal
        }
    }
}