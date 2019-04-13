extern crate rand;

use rand::Rng;

use std::io::Write;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;


include!("VectorLib.rs");
include!("traits.rs");
include!("hit_record.rs");
include!("ray.rs");
include!("sphere.rs");
include!("world.rs");
include!("camera.rs");
include!("material.rs");
include!("metal.rs");
include!("dielectric.rs");


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

    let cam: camera;
    cam = camera::init(Vec3::new(-2.0,2.0, 1.0),
                       Vec3::new(0.0,0.0, -1.0),
                       Vec3::new(0.0,1.0, 0.0),
                       90.0, (nx as f32)/ (ny as f32));
    let R:f32 = (std::f32::consts::PI/4.0).cos();

    let v1 = sphere {
        center: Vec3::new(-R, 0.0, -1.0),
        radius: R,
        material: Rc::new(lambertian {
            albedo: Vec3::new(0.0, 0.0, 1.0)
        }),
    };

    let v2 = sphere {
        center: Vec3::new(R, 0.0, -1.0),
        radius: R,
        material: Rc::new(lambertian {
            albedo: Vec3::new(1.0, 0.0, 0.0)
        }),
    };

    let v1 = sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(lambertian {
            albedo: Vec3::new(0.1, 0.2, 0.5)
        }),
    };

    let v2 = sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0)
        }),
    };

    let v3 = sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,

        material: Rc::new(metal::new(Vec3::new(0.8, 0.6, 0.2), 0.2)),
    };

    let v4 = sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,

        material: Rc::new(dielectric::new(1.5)),
    };

    let v5 = sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.45,

        material: Rc::new(dielectric::new(1.5)),
    };

    spheres.push(v1);
    spheres.push(v2);
    spheres.push(v3);
    spheres.push(v4);
    spheres.push(v5);
    let world = hitable_list::new(2, &spheres);

    while j >= 0 {
        for i in 0..nx {
            let mut colu = Vec3::new(0.0, 0.0, 0.0);
            for k in 0..ns {
                let i = i as f32;
                let j = j as f32;
                let rgbVec = Vec3 {
                    x: (i + drand48()) / (nx as f32),
                    y: (j + drand48()) / (ny as f32),
                    z: 0.2,
                };
                let c = cam.clone();
                let r = c.get_ray(rgbVec.x, rgbVec.y);
                let p = r.clone().point_at_parameter(2.0);
                let v = color(r.clone(), &world, 0);
                match v {
                    Some(v) => {
                        colu = colu + v
                    }
                    None => continue
                }
            }

            colu = colu / ns as f32;
            colu = Vec3::new(colu.x.sqrt(), colu.y.sqrt(), colu.z.sqrt());
            let ir = (255.99 * colu.x) as i32;
            let ig = (255.99 * colu.y) as i32;
            let ib = (255.99 * colu.z) as i32;

            let new_string = format!("{} {} {} \n", ir, ig, ib);
            actual_file.write(new_string.as_bytes());
        }

        j -= 1;
    }
}

fn color(ray: Ray, world: &hitable_list, depth: i32) -> Option<Vec3> {
    let tempVec3: Option<Vec3> = None;
    let (ray_hit_result, rec) = world.hit(&ray, 0.001, std::f32::MAX);
    if ray_hit_result {
        match rec {
            None => return tempVec3,
            Some(rec) => {
                let (res, scattered, attenuation) = rec.material.scatter(ray, &rec);
                if depth < 50 && res {
                    let c = color(scattered, &world, depth + 1);
                    match c {
                        None => return tempVec3,
                        Some(c) => {
                            return Some(attenuation * c);
                        }
                    }
                } else {
                    return Some(Vec3::new(0.0, 0.0, 0.0));
                }
            }
        }
    } else {
        let rayDirectionUnitVector = ray.direction.make_unit_vector();
        let t = 0.5 * (rayDirectionUnitVector.y + 1.0);
        return Some(Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t);
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = Vec3::new(drand48(), drand48(), drand48()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if p.length_squared() >= 1.0 {
            break;
        }
    }

    return p.clone();
}


pub fn drand48() -> f32 {
    let rand_float: f32 = rand::thread_rng().gen();
    rand_float
}
