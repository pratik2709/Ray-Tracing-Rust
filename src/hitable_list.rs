struct hitable_list <'a>{
    list_size:i32,
    list: &'a Vec<sphere>

}

impl<'a> hitable_list <'a>{
    fn new(list_size:i32, list: &'a Vec<sphere>) -> hitable_list{
        hitable_list{
            list_size,list
        }
    }
}

impl<'a> RayHit for hitable_list<'a>{
    fn hit(self, ray: Ray, t_min:f32, t_max: f32,  rec: hit_record) -> bool{
        let temp_rec: hit_record;
        let hit_anything = false;
        let closest_so_far: f64= t_max as f64;

        for i in 0..self.list_size{
            println!("{}", i);
//            let m = self.list[i];
        }
        true
    }
}

