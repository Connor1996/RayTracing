use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::util::random_usize_range;

pub struct BVH {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    bounding_box: AABB,
}

impl BVH {
    pub fn new(objects: &[Arc<dyn Hittable>]) -> Self {
        let cmp = match random_usize_range(0..3) {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => unreachable!(),
        };

        let left;
        let right;
        if objects.len() == 1 {
            left = Some(objects[0].clone());
            right = Some(objects[0].clone());
        } else if objects.len() == 2 {
            match cmp(&objects[0], &objects[1]) {
                Ordering::Less => {
                    left = Some(objects[0].clone());
                    right = Some(objects[1].clone());
                }
                _ => {
                    left = Some(objects[1].clone());
                    right = Some(objects[0].clone());
                }
            }
        } else {
            let mut objects = objects.to_vec();
            objects.sort_by(cmp);

            let mid = objects.len() / 2;
            left = Some(Arc::new(BVH::new(&objects[..mid])));
            right = Some(Arc::new(BVH::new(&objects[mid..])));
        }

        let bounding_box = AABB::surrounding_box(
            left.as_ref().unwrap().bounding_box().unwrap(),
            right.as_ref().unwrap().bounding_box().unwrap(),
        );

        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_range: &RangeInclusive<f64>) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_range) {
            return None;
        }

        let mut range = t_range.clone();
        let mut res = None;
        if let Some(l) = self.left.as_ref() {
            res = l.hit(ray, &range);
            if res.is_some() {
                range = RangeInclusive::new(*range.start(), res.as_ref().unwrap().t);
            }
        }

        if let Some(r) = self.right.as_ref() {
            let res1 = r.hit(ray, &range);
            if res1.is_some() {
                res = res1
            }
        }
        res
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box().unwrap();
    let box_b = b.bounding_box().unwrap();

    if box_a.minimum[axis] < box_b.minimum[axis] {
        Ordering::Less
    } else if box_a.minimum[axis] > box_b.minimum[axis] {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
