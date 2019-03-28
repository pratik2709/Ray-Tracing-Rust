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
    fn hit(self, ray: Ray, t_min:f32, t_max: f32) -> bool{
        let mut hit_anything = false;
        let mut closest_so_far = t_max ;

        for i in self.list{
            let ii = i.clone();
            let new_ray = ray.clone();
            let n:bool= ii.hit(new_ray, t_min, closest_so_far);
            if n{
                hit_anything=true;
            }
        }
        hit_anything
    }
}

