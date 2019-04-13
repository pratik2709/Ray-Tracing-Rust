#[derive(Clone)]
pub struct camera{
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u:Vec3,
    v:Vec3,
    w:Vec3,
    lens_radius:f32,
}

impl camera{

    fn init(lookfrom:Vec3, lookat:Vec3, vup:Vec3
            , vfov:f32, aspect:f32, aperture:f32, focus_dist:f32) -> camera{
        let lens_radius = aperture/2.0;
        let u;
        let v;
        let w;
        let M_PI = std::f32::consts::PI;
        let theta = vfov*(M_PI)/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom.clone();
        w = (lookfrom - lookat).make_unit_vector();
        u = vup.cross(&w);
        v = w.clone().cross(&u);
//        let lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
        let lower_left_corner = origin.clone()
            - u.clone()*half_width*focus_dist
            - v.clone()*half_height*focus_dist
            - w.clone()*focus_dist;

        let horizontal=u.clone()*2.0* half_width*focus_dist;
        let vertical= v.clone()*2.0* half_height*focus_dist;
        camera{
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
            lens_radius
        }

    }

    fn get_ray(self, s:f32, t:f32) -> Ray{
        let rd: Vec3 = random_in_unit_disk()*self.lens_radius;
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin.clone() + offset.clone(),
                 self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin
                     .clone() -offset)
    }

}


fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = Vec3::new(drand48(), drand48(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(&p) >= 1.0 {
            continue;
        }
        else{
            break;
        }
    }

    return p.clone();
}