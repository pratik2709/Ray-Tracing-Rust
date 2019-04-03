#[derive(Clone)]
struct lambertian {
    albedo: Vec3,
}

impl lambertian {
    fn new(mut self, a: Vec3) {
        self.albedo = Vec3{
            x: a.x,
            y: a.y,
            z: a.z,
        }
    }
}

impl Material for lambertian {
    fn scatter(&self, ray: Ray, rec: hit_record) -> (bool, Ray, Vec3) {
        let target: Vec3 = rec.getP() + rec.getNormal() + random_in_unit_sphere();
        let scattered = Ray::new(rec.getP(), target - rec.getP());
        let res = scattered.direction.dot(&rec.normal) > 0.0;
        (res, scattered, self.albedo.clone())
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    let temp1 = n.clone() * (v.dot(&n));
    v - temp1 * 2.0
}