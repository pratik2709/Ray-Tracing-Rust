#[derive(Clone)]
struct material{

}

struct lambertian{
    superclass: material,
    albedo: Vec3
}

impl lambertian{
    fn new(&self, a:Vec3){
        self.albedo = a
    }
}

impl Material for lambertian{
    fn scatter(&self, ray: Ray, scattered: Ray, rec: hit_record, attentuation: Vec3) -> bool{
        let target : Vec3 = rec.getP() + rec.getNormal() + random_in_unit_sphere();
        scattered = Ray::new(rec.getP(), target - rec.getP());
        attentuation = self.albedo.clone();
        scattered.direction.dot(&rec.normal) > 0.0
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3{
    let temp1 = n*(v.dot(&n));
    v - temp1*2.0
}