#[derive(Clone)]
pub struct camera{
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl camera{

    fn init(lookfrom:Vec3, lookat:Vec3, vup:Vec3, vfov:f32, aspect:f32) -> camera{
        let u;
        let v;
        let w;
        let M_PI = std::f32::consts::PI;
        let theta = vfov*(M_PI)/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom.clone();
        w = (lookfrom - lookat).make_unit_vector();
        u = vup.cross(w.clone());
        v = w.clone().cross(u.clone());
//        let lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
        let lower_left_corner = origin.clone() - u.clone()*half_width - v.clone()*half_height -
            w.clone();

        let horizontal=u*2.0* half_width;
        let vertical= v*2.0* half_height;
        camera{
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }

    }

    fn get_ray(self, u:f32, v:f32) -> Ray{
        Ray::new(self.origin.clone(),
                 self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin
                     .clone())
    }
}