struct metal{
    albedo: Vec3,
}

impl Material for metal {
    fn scatter(&self, ray: Ray, rec: &hit_record) -> (bool, Ray, Vec3) {
        let reflected: Vec3 = reflect((ray.getDirection().make_unit_vector()), rec.getNormal());
        let scattered = Ray::new(rec.getP(), reflected);

        let res = scattered.direction.dot(&rec.normal) > 0.0;
        (res, scattered, self.albedo.clone())
    }

}