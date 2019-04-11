struct dielectric{
    ref_idx:f32
}

impl dielectric{
    fn new(ref_idx:f32) -> dielectric{
        dielectric{
            ref_idx
        }
    }
}

impl Material for dielectric {
    fn scatter(&self, ray: Ray, rec: &hit_record) -> (bool, Ray, Vec3) {
        let outward_normal: Vec3;
        let reflected = reflect(ray.getDirection(), rec.getNormal());
        let ni_over_nt:f32;
        let attenuation = Vec3::new(1.0,1.0,1.0);
        let reflect_prob:f32;
        let cosine:f32;
        let mut scattered: Ray;

        if ray.getDirection().dot(&rec.getNormal()) > 0.0{
            outward_normal = -rec.getNormal();
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray.getDirection().dot(&rec.getNormal()) / ray.getDirection()
                .length();
        }
        else{
            outward_normal = rec.getNormal();
            ni_over_nt = 1.0/self.ref_idx;
            cosine = (-1.0 * ray.getDirection().dot(&rec.getNormal())) / ray.getDirection()
                .length();
        }


        let (b, refracted) = refract(ray.getDirection(), outward_normal, ni_over_nt);
        if b {
            reflect_prob = schlick(cosine, self.ref_idx);
        }
        else{
            scattered = Ray::new(rec.getP(), reflected.clone());
            reflect_prob = 1.0;
        }

        if drand48() < reflect_prob {
            scattered = Ray::new(rec.getP(), reflected);
        }
        else{
            match refracted {
                Some(refracted) => {
                    scattered = Ray::new(rec.getP(), refracted);
                }
                None => panic!("empty!")
            }

        }

        (true, scattered, attenuation)

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


fn schlick(cosine:f32, ref_idx:f32) -> f32{
    let mut r0:f32 = (1.0- ref_idx)/(1.0+ref_idx);
    r0 = r0*r0;
    let temp:f32 = (1.0 - cosine);
    r0 + (1.0 - r0)*temp.powf(5.0)
}