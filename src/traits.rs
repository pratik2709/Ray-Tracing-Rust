trait RayHit{
    fn hit(&self, ray: &Ray, t_min:f32, t_max: f32) -> (bool, Option<hit_record>);
}

trait Material{
    fn scatter(&self, ray: Ray, scattered: Ray, attentuation: Vec3) -> (bool, Option<hit_record>);
}