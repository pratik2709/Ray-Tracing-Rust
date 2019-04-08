fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> (bool, Option<Vec3>){
    let uv: Vec3 = v.make_unit_vector();
    let dt:f32 = uv.dot(&n);
    let discriminant:f32 = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    let mut refracted:Option<Vec3> = None;
    if discriminant > 0.0 {
        refracted = Some( (uv - n.clone()*dt)*ni_over_nt - n.clone()*discriminant.sqrt());
        (true, refracted)
    }
    else{
        (false, refracted)
    }
}