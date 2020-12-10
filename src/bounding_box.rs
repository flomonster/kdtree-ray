use cgmath::*;

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

impl AABB {
    pub(crate) fn intersect_ray(
        &self,
        ray_origin: &Vector3<f32>,
        ray_direction: &Vector3<f32>,
    ) -> bool {
        let mut tmin = (self.0.x - ray_origin.x) / ray_direction.x;
        let mut tmax = (self.1.x - ray_origin.x) / ray_direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.0.y - ray_origin.y) / ray_direction.y;
        let mut tymax = (self.1.y - ray_origin.y) / ray_direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        tmin = tmin.max(tymin);
        tmax = tmax.min(tymax);

        let mut tzmin = (self.0.z - ray_origin.z) / ray_direction.z;
        let mut tzmax = (self.1.z - ray_origin.z) / ray_direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        true
    }

    pub(crate) fn intersect_box(&self, other: &AABB) -> bool {
        (self.0.x < other.1.x && self.1.x > other.0.x)
            && (self.0.y < other.1.y && self.1.y > other.0.y)
            && (self.0.z < other.1.z && self.1.z > other.0.z)
    }
}
