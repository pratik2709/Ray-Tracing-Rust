use std::io::Write;
use std::error::Error;
use std::fs::File;
use std::path::Path;

include!("VectorLib.rs");
include!("ray.rs");


fn main(){

    let actual_path = Path::new("output.txt");
    let mut actual_file = match File::create(&actual_path) {
        Err(why) => panic!("Something went wrong:: {}", why.description()),
        Ok(actual_file) => actual_file,
    };

    let nx = 200;
    let ny = 100;
    let first_line = format!("{}{} {} {}", "P3\n", nx , ny , "\n255\n");
    actual_file.write(first_line.as_bytes());

    let mut j = ny-1;
    while j>= 0 {
        for i in 0..nx{
            let i = i as f32;
            let j = j as f32;

            let rgbVec = Vec3{
                x: i/(nx as f32),
                y: j/(ny as f32),
                z: 0.2
            };

            let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
            let horizontal = Vec3::new(4.0, 0.0, 0.0);
            let vertical = Vec3::new(0.0, 2.0, 0.0);
            let origin = Vec3::new(0.0, 0.0, 0.0);
            let ray:Ray = Ray::new(origin,
                lower_left_corner + horizontal*rgbVec.x + vertical*rgbVec.y);

            let v = color(ray);

            let ir = (255.99 * v.x) as i32;
            let ig = (255.99 * v.y) as i32;
            let ib = (255.99 * v.z) as i32;

            let new_string = format!("{} {} {} \n", ir,ig, ib);
            actual_file.write(new_string.as_bytes());

        }

        j -= 1;
    }

}

fn color(ray: Ray) -> Vec3 {
    let v:Vec3;
    if hit_sphere(Vec3::new(0.0,0.0,-1.0),0.5, &ray){
        v = Vec3::new(1.0,0.0,0.0)
    }
    else{
        let rayDirectionUnitVector = ray.direction.make_unit_vector();
        let t = 0.5 * (rayDirectionUnitVector.y + 1.0);
        v = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }

    v
}

fn hit_sphere(center: Vec3, radius: f32, ray:&Ray) -> bool{
    let or = ray.getOrigin();
    let oc = or - center;
    let tempRay = & ray.direction;
    let a = tempRay.dot(tempRay);
    let b = 2.0 * oc.dot(tempRay);
    let c = &oc.dot(&oc) - radius*radius;
    let discriminant = b*b - (a*c)*4.0;
    discriminant > 0.0
}
