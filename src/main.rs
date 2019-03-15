fn main(){
    let nx = 200;
    let ny = 100;
    println!("{}{} {} {}", "P3\n", nx , ny , "\n255\n");

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
            println!("{} {} {} \n", ir,ig, ib);

        }

        j -= 1;
    }

}