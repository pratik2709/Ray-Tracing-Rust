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
    fn hit(self, ray: Ray, t_min: f32, t_max: f32) -> (bool, Option<hit_record>) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        let rec_ret: Option<hit_record> = None;
        for i in self.list {
            let ii = i.clone();
            let new_ray = ray.clone();
            let (n, rec) = ii.hit(new_ray, t_min, closest_so_far);

            match rec {
                None => return (hit_anything, rec),
                Some(rec) => if n {
                    hit_anything = true;
                    return (hit_anything, Some(rec))
                }
            }
        }
        (hit_anything, rec_ret)
    }
}

