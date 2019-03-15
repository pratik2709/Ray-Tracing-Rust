use std::io::Write;
use std::error::Error;
use std::fs::File;
use std::path::Path;

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
            let r:f32 = i/(nx as f32);
            let g:f32 = j/(ny as f32);
            let b:f32 = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            let new_string = format!("{} {} {} \n", ir,ig, ib);
            actual_file.write(new_string.as_bytes());

        }

        j -= 1;
    }

}