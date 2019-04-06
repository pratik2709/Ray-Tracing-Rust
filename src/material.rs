#[derive(Clone)]
struct lambertian {
    albedo: Vec3,
}


impl Material for lambertian {
    fn scatter(&self, ray: Ray, rec: &hit_record) -> (bool, Ray, Vec3) {
        let target: Vec3 = rec.getP() + rec.getNormal() + random_in_unit_sphere();
        let scattered = Ray::new(rec.getP(), target - rec.getP());
        let res = scattered.direction.dot(&rec.normal) > 0.0;
        (res, scattered, self.albedo.clone())
    }

//    fn clone(&self) -> Box<Material>{
//        Box::new(lambertian{albedo:self.albedo})
//    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    let temp1 = n.clone() * (v.dot(&n));
    v - temp1 * 2.0
}