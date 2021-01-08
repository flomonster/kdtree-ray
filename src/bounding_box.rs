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
pub type AABB = [Vector3<f32>; 2];

/// BoundingBox trait is needed to use a KD-tree.
pub trait BoundingBox {
    /// This function return the **Axis-aligned bounding boxes**
    /// (`AABB`) of the object.
    ///
    /// For more information check [AABB](type.AABB.html).
    fn bounding_box(&self) -> AABB;
}
