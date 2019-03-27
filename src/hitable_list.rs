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
        let mut hit_anything = false;
        let mut closest_so_far: f64= t_max as f64;

        for i in self.list{
            println!("{:?}", i);
            let n:bool= i.hit(ray, t_min, closest_so_far, temp_rec);
            if n{
                hit_anything=true;
                closest_so_far = temp_rec.t as f64;
                rec = temp_rec;
            }
        }
        hit_anything
    }
}

