struct dielectric{
    ref_idx:f32
}

impl Material for dielectric {
    fn scatter(&self, ray: Ray, rec: &hit_record) -> (bool, Ray, Vec3) {
        let outward_normal: Vec3;
        let reflected = reflect(ray.getDirection(), rec.getNormal());
        let ni_over_nt:f32;
        let attenuation = Vec3::new(1.0,1.0,1.0);
//        let refracted:Vec3;
        if ray.getDirection().dot(rec.getNormal()) > 0{
            outward_normal = -rec.getNormal();
            ni_over_nt = self.ref_idx;
        }
        else{
            outward_normal = rec.getNormal();
            ni_over_nt = 1.0/self.ref_idx;
        }

        let (b, refracted) = refract(ray.getDirection(), outward_normal, ni_over_nt);
        if b {

        }
    }

}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> (bool, Option<Vec3>){
    let uv: Vec3 = v.make_unit_vector();
    let dt:f32 = uv.dot(&n);
    let discriminant:f32 = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    let mut refracted:Option<Vec3> = None;
    if discriminant > 0.0 {
        refracted = Some( (uv - n.clone()*dt)*ni_over_nt - n.clone()*discriminant.sqrt());
        (true, refracted)
    }
    else{
        (false, refracted)
    }
}