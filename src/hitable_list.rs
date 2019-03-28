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
    fn hit(self, ray: Ray, t_min:f32, t_max: f32,  mut rec: &mut hit_record) -> bool{
        let mut temp_rec: &mut hit_record;
//        temp_rec = rec;
        let mut hit_anything = false;
        let mut closest_so_far = t_max ;

        for i in self.list{
            let ii = i.clone();
            let new_ray = ray.clone();
//            let new_temp_rec = temp_rec.clone();
            let n:bool= ii.hit(new_ray, t_min, closest_so_far, temp_rec);
            if n{
                hit_anything=true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }
        hit_anything
    }
}

