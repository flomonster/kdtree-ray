use crate::AABB;
use cgmath::*;

pub(crate) struct Ray {
    origin: Vector3<f32>,
    inv_direction: Vector3<f32>,
    sign: Vector3<usize>,
}

impl Ray {
    pub fn new(origin: &Vector3<f32>, direction: &Vector3<f32>) -> Self {
        let inv_direction = Vector3::new(1. / direction.x, 1. / direction.y, 1. / direction.z);
        let sign = Vector3::new(
            (direction.x < 0.) as usize,
            (direction.y < 0.) as usize,
            (direction.z < 0.) as usize,
        );

        Self {
            origin: origin.clone(),
            inv_direction,
            sign,
        }
    }
    pub fn intersect(&self, aabb: &AABB) -> bool {
        let mut ray_min = (aabb[self.sign.x].x - self.origin.x) * self.inv_direction.x;
        let mut ray_max = (aabb[1 - self.sign.x].x - self.origin.x) * self.inv_direction.x;

        let y_min = (aabb[self.sign.y].y - self.origin.y) * self.inv_direction.y;
        let y_max = (aabb[1 - self.sign.y].y - self.origin.y) * self.inv_direction.y;

        if (ray_min > y_max) || (y_min > ray_max) {
            return false;
        }

        if y_min > ray_min {
            ray_min = y_min;
        }
        // Using the following solution significantly decreases the performance
        // ray_min = ray_min.max(y_min);

        if y_max < ray_max {
            ray_max = y_max;
        }
        // Using the following solution significantly decreases the performance
        // ray_max = ray_max.min(y_max);

        let z_min = (aabb[self.sign.z].z - self.origin.z) * self.inv_direction.z;
        let z_max = (aabb[1 - self.sign.z].z - self.origin.z) * self.inv_direction.z;

        if (ray_min > z_max) || (z_min > ray_max) {
            return false;
        }

        if z_max < ray_max {
            ray_max = z_max;
        }

        // Using the following solution significantly decreases the performance
        // ray_max = ray_max.min(y_max);

        ray_max > 0.0
    }
}
