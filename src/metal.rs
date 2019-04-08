struct metal{
    albedo: Vec3,
    fuzz: f32
}

impl Material for metal {

    fn scatter(&self, ray: Ray, rec: &hit_record) -> (bool, Ray, Vec3) {
        let reflected: Vec3 = reflect((ray.getDirection().make_unit_vector()), rec.getNormal());
        let scattered = Ray::new(rec.getP(), reflected + random_in_unit_sphere()*self.fuzz);

        let res = scattered.direction.dot(&rec.normal) > 0.0;
        (res, scattered, self.albedo.clone())
    }

}

impl metal{
    fn new(a:Vec3, f:f32) -> metal{
        let fuzz:f32;
        if f< 1.0{
            fuzz = f;
        }
        else{
            fuzz = 1.0;
        }
        metal{
            albedo: a,
            fuzz
        }
    }
}