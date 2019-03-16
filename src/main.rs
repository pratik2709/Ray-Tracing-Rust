use std::io::Write;
use std::error::Error;
use std::fs::File;
use std::path::Path;

include!("VectorLib.rs");


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


            let ir = (255.99 * rgbVec.x) as i32;
            let ig = (255.99 * rgbVec.y) as i32;
            let ib = (255.99 * rgbVec.z) as i32;
            let new_string = format!("{} {} {} \n", ir,ig, ib);
            actual_file.write(new_string.as_bytes());

        }

        j -= 1;
    }

}