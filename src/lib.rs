#![deny(missing_docs)]

//! This crate is a fast implementation of [KD-tree](https://en.wikipedia.org/wiki/K-d_tree)
//! for raytracer (or other rendering method using ray).
//!
//! It's based on this [paper](http://www.irisa.fr/prive/kadi/Sujets_CTR/kadi/Kadi_sujet2_article_Kdtree.pdf)
//! written by *Ingo Wald* and *Vlastimil Havran*.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! kdtree-ray="0.1.0"
//! ```
//!
//! # Usage & Tips
//!
//! To create a [KD-tree](struct.KDtree.html) you only need to implement
//! the [BoundingBox](trait.BoundingBox.html) on the object.
//!
//! If you're doing a raytracer each mesh could contain a KD-tree of triangles.
//! Since `KDtree` his implementing `BoundingBox` itself you can create a KDtree
//! of meshes in your scene.
//!
//! # Example
//!
//! ```
//! use cgmath::*;
//! use kdtree_ray::{AABB, BoundingBox, KDtree};
//! struct Triangle(Vector3<f32>, Vector3<f32>, Vector3<f32>);
//!
//! // To use the KDtree on an object you need first to implement the BoundingBox trait.
//! impl BoundingBox for Triangle {
//!   fn bounding_box(&self) -> AABB {
//!     let min = Vector3::new(
//!       self.0.x.min(self.1.x).min(self.2.x),
//!       self.0.y.min(self.1.y).min(self.2.y),
//!       self.0.z.min(self.1.z).min(self.2.z),
//!     );
//!     let max = Vector3::new(
//!       self.0.x.max(self.1.x).max(self.2.x),
//!       self.0.y.max(self.1.y).max(self.2.y),
//!       self.0.z.max(self.1.z).max(self.2.z),
//!     );
//!     AABB(min, max)
//!   }
//! }
//!
//! fn main() {
//!   // Kdtree creation
//!   let triangles: Vec<Triangle> = vec![/* ... */];
//!   let kdtree = KDtree::new(triangles);
//!
//!   // Get a reduced list of triangles that a ray could intersect
//!   let ray_origin = Vector3::zero();
//!   let ray_direction = Vector3::new(1., 0., 0.);
//!   let candidates_triangles = kdtree.intersect(&ray_origin, &ray_direction);
//! }
//! ```
mod bounding_box;
mod candidate;
mod item;
mod kdnode;
mod plane;
mod side;

pub use bounding_box::*;

use candidate::{Candidate, Candidates};
use cgmath::*;
use item::Item;
use kdnode::KDtreeNode;
use side::Side;
use std::collections::HashSet;
use std::sync::Arc;

/// The KD-tree data structure stores the elements implementing BoundingBox.
#[derive(Clone, Debug)]
pub struct KDtree<P>
where
    P: BoundingBox,
{
    root: KDtreeNode<P>,
    space: AABB,
}

impl<P: BoundingBox> KDtree<P> {
    /// This function is used to create a new KD-tree. You need to provide a
    /// `Vec` of values that implement `BoundingBox` trait.
    pub fn new(mut values: Vec<P>) -> Self {
        let mut space = AABB(Vector3::<f32>::max_value(), Vector3::<f32>::min_value());
        let n = values.len();
        let mut candidates = Candidates::with_capacity(n * 6);
        for (id, v) in values.drain(..).enumerate() {
            // Create items from values
            let bb = v.bounding_box();
            let item = Arc::new(Item::new(v, id));
            candidates.append(&mut Candidate::gen_candidates(item, &bb));

            // Update space with the bounding box of the item
            space.0.x = space.0.x.min(bb.0.x);
            space.0.y = space.0.y.min(bb.0.y);
            space.0.z = space.0.z.min(bb.0.z);
            space.1.x = space.1.x.max(bb.1.x);
            space.1.y = space.1.y.max(bb.1.y);
            space.1.z = space.1.z.max(bb.1.z);
        }

        // Sort candidates only once at the begining
        candidates.sort_by(|a, b| a.cmp(&b));

        // Will be used to classify candidates
        let mut sides = vec![Side::Both; n];
        let root = KDtreeNode::new(&space, candidates, n, &mut sides);
        KDtree { space, root }
    }

    /// This function takes a ray and return a reduced list of candidates that
    /// can be intersected by the ray.
    pub fn intersect(
        &self,
        ray_origin: &Vector3<f32>,
        ray_direction: &Vector3<f32>,
    ) -> Vec<Arc<P>> {
        // Check if the ray intersect the bounding box of the Kd Tree
        if self.space.intersect_ray(ray_origin, ray_direction) {
            let mut items = HashSet::new();
            self.root.intersect(ray_origin, ray_direction, &mut items);
            items.iter().map(|e| e.value.clone()).collect()
        } else {
            vec![]
        }
    }
}

impl<P> BoundingBox for KDtree<P>
where
    P: BoundingBox,
{
    fn bounding_box(&self) -> AABB {
        self.space.clone()
    }
}
