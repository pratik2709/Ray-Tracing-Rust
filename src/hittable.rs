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

trait RayHit{
    fn hit(self, ray: Ray, t_min:f32, t_max: f32) -> (bool, hit_record);
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
    fn hit(self, ray: Ray, t_min:f32, t_max: f32) -> (bool, hit_record){
        let oc = ray.getOrigin() - self.center();
        let a:f32 = ray.getDirection().dot(&ray.direction);
        let b:f32 = ray.getDirection().dot(&oc);
        let c:f32 = oc.dot(&oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        let rec: hit_record;
        if discriminant > 0.0 {
            let mut temp:f32 = (-b-(b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(t);
                let normal = (p.clone() - self.center())/self.radius;
                rec = hit_record::new(t,p,normal);
                return (true, rec)
            }
            temp = (-b + (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(t);
                let normal = (p.clone() - self.center())/self.radius;
                rec = hit_record::new(t,p,normal);
                return (true, rec)
            }
        }
        (false, rec)
    }
}