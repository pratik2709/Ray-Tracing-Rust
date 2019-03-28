#[derive(Clone)]
pub struct hit_record{
    pub t:f32,
    pub p:Vec3,
    pub normal:Vec3
}

//pub struct hitable{
//
//}
//
//impl hitable{
//    fn hit(self, ray: Ray, t_min:f32, t_max: f32, rec:hit_record) -> bool;
//}

trait RayHit{
    fn hit(self, ray: Ray, t_min:f32, t_max: f32) -> bool;
}

#[derive(Debug)]
#[derive(Clone)]
struct sphere{
    center: Vec3,
    radius: f32,
}

impl sphere{
    fn center(&self) -> Vec3{
        Vec3 {
            x:self.center.x,
            y:self.center.y,
            z:self.center.z,
        }
    }
}

impl RayHit for sphere{
    fn hit(self, ray: Ray, t_min:f32, t_max: f32) -> bool{
        let oc = ray.getOrigin() - self.center();
        let a:f32 = ray.getDirection().dot(&ray.direction);
        let b:f32 = ray.getDirection().dot(&oc);
        let c:f32 = oc.dot(&oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp:f32 = (-b-(b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {

                return true
            }
            temp = (-b + (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {

                return true
            }
        }
        false
    }
}