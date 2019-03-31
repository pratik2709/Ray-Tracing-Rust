#[derive(Clone)]
struct hitable_list<'a> {
    list_size: i32,
    list: &'a Vec<sphere>,
}

impl<'a> hitable_list<'a> {
    fn new(list_size: i32, list: &'a Vec<sphere>) -> hitable_list {
        hitable_list {
            list_size,
            list,
        }
    }
}

impl<'a> RayHit for hitable_list<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> (bool, Option<hit_record>) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        let mut rec: Option<hit_record> = None;
        for i in self.list {
            let ii = i.clone();
            let new_ray = ray.clone();
            let (n, new_rec) = ii.hit(&ray, t_min, closest_so_far);

            if n {
                match new_rec {
                    None => {
                        hit_anything = true;
                        return (hit_anything, new_rec)
                    },
                    Some(new_rec) => {
                        hit_anything = true;
                        closest_so_far = new_rec.t;
                        let n_new_rec = new_rec.clone();
                        rec = Some(n_new_rec);
                    }
                }
            }

        }
        (hit_anything, rec)
    }
}

