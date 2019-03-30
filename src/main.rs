extern crate rand;
use rand::Rng;

use std::io::Write;
use std::error::Error;
use std::fs::File;
use std::path::Path;



include!("VectorLib.rs");
include!("ray.rs");
include!("hittable.rs");
include!("hitable_list.rs");
include!("camera.rs");


fn main() {
    let actual_path = Path::new("output.txt");
    let mut actual_file = match File::create(&actual_path) {
        Err(why) => panic!("Something went wrong:: {}", why.description()),
        Ok(actual_file) => actual_file,
    };

    let nx = 200;
    let ny = 100;
    let ns = 100;
    let first_line = format!("{}{} {} {}", "P3\n", nx, ny, "\n255\n");
    actual_file.write(first_line.as_bytes());

    let mut j = ny - 1;

    let mut spheres: Vec<sphere> = Vec::new();
    let v1 = sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let v2 = sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };
    spheres.push(v1);
    spheres.push(v2);
    let world = hitable_list::new(2, &spheres);


    while j >= 0 {
        for i in 0..nx {
            let mut colu = Vec3::new(0.0,0.0,0.0);
            for k in 0 .. ns{

                let i = i as f32;
                let j = j as f32;
                let rgbVec = Vec3 {
                    x: (i + drand48()) / (nx as f32),
                    y: (j + drand48()) / (ny as f32),
                    z: 0.2
                };
                let cam: camera;
                cam = camera::init();
                let r = cam.get_ray(rgbVec.x, rgbVec.y);
                let p = r.clone().point_at_parameter(2.0);
                let new_world = world.clone();
                let v = color(r.clone(), new_world);
                match v {
                    Some(v) => {
                        colu = colu +v
                    },
                    None => continue
                }
            }

            colu = colu/ns as f32;
            let ir = (255.99 * colu.x) as i32;
            let ig = (255.99 * colu.y) as i32;
            let ib = (255.99 * colu.z) as i32;

            let new_string = format!("{} {} {} \n", ir, ig, ib);
            actual_file.write(new_string.as_bytes());


        }

        j -= 1;
    }
}

fn color(ray: Ray, world: hitable_list) -> Option<Vec3> {
    let tempVec3: Option<Vec3> = None;
    let (ray_hit_result, rec) = world.hit(ray.clone(), 0.0, std::f32::MAX);
    if ray_hit_result {
        match rec {
            None => return tempVec3,
            Some(rec) => return Some(Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0) * 0.5),
        }
    }
    else{
        let rayDirectionUnitVector = ray.direction.make_unit_vector();
        let t = 0.5 * (rayDirectionUnitVector.y + 1.0);
        return Some(Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t);
    }

}


fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let or = ray.getOrigin();
    let oc = or - center;
    let tempRay = &ray.direction;
    let a = tempRay.dot(tempRay);
    let b = 2.0 * oc.dot(tempRay);
    let c = &oc.dot(&oc) - radius * radius;
    let discriminant = b * b - (a * c) * 4.0;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

pub fn drand48() -> f32 {
    let rand_float : f32 = rand::thread_rng().gen();
    rand_float
}
