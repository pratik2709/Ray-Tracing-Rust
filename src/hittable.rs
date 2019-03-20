pub struct hit_record{
    pub t:f32,
    pub p:Vec3,
    pub normal:Vec3
}

trait RayHit{
    fn hit(self, ray: Ray, t_min:f32, t_max: f32, rec:hit_record) -> bool;
}

struct sphere{
    center: Vec3,
    radius: f32,
}

impl RayHit for sphere{
    fn hit(self, ray: Ray, t_min:f32, t_max: f32, mut rec: hit_record) -> bool{
        let oc = ray.getOrigin() - self.center;
        let a:f32 = ray.getDirection().dot(&ray.direction);
        let b:f32 = ray.getDirection().dot(&oc);
        let c:f32 = oc.dot(&oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp:f32 = (-b-(b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - center)/radius;
                true
            }
            temp = (-b + (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = ray.point_at_parameter(rec.t);
                rec.normal = (rec.p - center)/radius;
                true
            }
        }
        false
    }
}