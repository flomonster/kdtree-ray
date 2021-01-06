use cgmath::*;
use std::ops::Index;

/// Axis-aligned bounding box is defined by two positions.
///
/// **Note**: The first position is expected to be the minimum bound and the second
/// the maximum bound.
///
/// The animated GIF below shows a graphic example of an AABB that adapts its
/// size to fit the rotating entity. The box constantly changes dimensions to
/// snugly fit the entity contained inside.
///
/// ![Gif describing an AABB](https://media.prod.mdn.mozit.cloud/attachments/2015/10/16/11799/57dfaf5508784d6b9c5fe77c0df49a54/rotating_knot.gif)
#[derive(Clone, Debug)]
pub struct AABB(pub Vector3<f32>, pub Vector3<f32>);

/// BoundingBox trait is needed to use a KD-tree.
pub trait BoundingBox {
    /// This function return the **Axis-aligned bounding boxes**
    /// (`AABB`) of the object.
    ///
    /// For more information check [AABB](type.AABB.html).
    fn bounding_box(&self) -> AABB;
}

impl Index<usize> for AABB {
    type Output = Vector3<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            _ => &self.1,
        }
    }
}

impl AABB {
    pub(crate) fn intersect_ray(
        &self,
        origin: &Vector3<f32>,
        inv_direction: &Vector3<f32>,
        sign: &Vector3<usize>,
    ) -> bool {
        let mut ray_min = (self[sign.x].x - origin.x) * inv_direction.x;
        let mut ray_max = (self[1 - sign.x].x - origin.x) * inv_direction.x;

        let y_min = (self[sign.y].y - origin.y) * inv_direction.y;
        let y_max = (self[1 - sign.y].y - origin.y) * inv_direction.y;

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

        let z_min = (self[sign.z].z - origin.z) * inv_direction.z;
        let z_max = (self[1 - sign.z].z - origin.z) * inv_direction.z;

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
