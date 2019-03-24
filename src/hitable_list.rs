struct hitable_list <'a>{
    list_size:i32,
    list: &'a Vec3

}

impl<'a> RayHit for hitable_list<'a>{
    fn hit(self, ray: Ray, t_min:f32, t_max: f32, mut rec: hit_record) -> bool{
        true
    }
}