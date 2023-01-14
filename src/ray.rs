use crate::{Point3, Vector3, AABB};

/// A 3D ray
pub struct Ray {
    /// The origin of the ray
    origin: Vector3,
    /// The inverse of the direction of the ray (1 / direction)
    inv_direction: Vector3,
    /// The sign of the direction of the ray (0 if negative, 1 if positive)
    sign: [bool; 3],
}

impl Ray {
    pub fn new(origin: &Vector3, direction: &Vector3) -> Self {
        let inv_direction = Vector3::new(1. / direction.x, 1. / direction.y, 1. / direction.z);
        let sign = [direction.x < 0., direction.y < 0., direction.z < 0.];

        Self {
            origin: *origin,
            inv_direction,
            sign,
        }
    }

    fn get_aabb_sign(aabb: &AABB, sign: bool) -> Point3 {
        if sign {
            aabb.max
        } else {
            aabb.min
        }
    }

    pub fn intersect(&self, aabb: &AABB) -> bool {
        let mut ray_min =
            (Self::get_aabb_sign(aabb, self.sign[0]).x - self.origin.x) * self.inv_direction.x;
        let mut ray_max =
            (Self::get_aabb_sign(aabb, !self.sign[0]).x - self.origin.x) * self.inv_direction.x;

        let y_min =
            (Self::get_aabb_sign(aabb, self.sign[1]).y - self.origin.y) * self.inv_direction.y;
        let y_max =
            (Self::get_aabb_sign(aabb, !self.sign[1]).y - self.origin.y) * self.inv_direction.y;

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

        let z_min =
            (Self::get_aabb_sign(aabb, self.sign[2]).z - self.origin.z) * self.inv_direction.z;
        let z_max =
            (Self::get_aabb_sign(aabb, !self.sign[2]).z - self.origin.z) * self.inv_direction.z;

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
