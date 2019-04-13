#[derive(Clone)]
pub struct camera{
    theta: f32,
    half_height: f32,
    half_width: f32,
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl camera{

    fn init(vfov:f32, aspect:f32) -> camera{
        let M_PI = std::f32::consts::PI;
        let theta = vfov*(M_PI)/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        camera{
            theta,
            half_height,
            half_width,
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
            horizontal:Vec3::new(2.0*half_width, 0.0, 0.0),
            vertical:Vec3::new(0.0, 2.0*half_height, 0.0),
            origin:Vec3::new(0.0, 0.0, 0.0),
        }

    }

    fn get_ray(self, u:f32, v:f32) -> Ray{
        Ray::new(self.origin.clone(),
                 self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin
                     .clone())
    }
}