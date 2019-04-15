fn random_scene() -> Vec<sphere> {
    let n = 500;
    let mut spheres: Vec<sphere> = Vec::new();
    let s1 = sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Rc::new(lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5)
        }),
    };
    spheres.push(s1);

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = drand48();
            let center: Vec3 = Vec3::new(i as f32 + 0.9 * drand48(), 0.2, j as f32 + 0.9 * drand48
                ());
            if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let s2 = sphere {
                        center,
                        radius: 0.2,
                        material: Rc::new(lambertian {
                            albedo: Vec3::new(drand48() * drand48()
                                              , drand48() * drand48(),
                                              drand48() * drand48())
                        }),
                    };
                    spheres.push(s2);
                }
                else if choose_mat < 0.95  {
                    let s3 = sphere {
                        center,
                        radius: 0.2,
                        material: Rc::new(metal::new(Vec3::new(0.5 * (1.0 + drand48()),
                                                               0.5 * (1.0 + drand48()),
                                                               0.5 * (1.0 + drand48())),0.5*drand48())),
                    };
                    spheres.push(s3);
                }
                else{
                     let s4 = sphere {
                        center,
                        radius: 0.2,
                        material: Rc::new(dielectric::new(1.5)),
                    };
                    spheres.push(s4);
                }
            }
        }
    }

    let s6 = sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(dielectric::new(1.5)),
    };

    let s5 = sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1)
        }),
    };

    let s7 = sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,

        material: Rc::new(metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    };

    spheres.push(s5);
    spheres.push(s6);
    spheres.push(s7);
    spheres
}