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
//! kdtree-ray="1.0.0"
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
//! use kdtree_ray::{AABB, Bounded, KDTree};
//! struct Triangle(Vector3<f32>, Vector3<f32>, Vector3<f32>);
//!
//! // To use the KDTree on an object you need first to implement the BoundingBox trait.
//! impl Bounded for Triangle {
//!   fn bound(&self) -> AABB {
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
//!     AABB::new(min, max)
//!   }
//! }
//!
//! // Kdtree creation
//! let triangle = Triangle(Vector3::zero(), Vector3::zero(), Vector3::zero());
//! let triangles: Vec<Triangle> = vec![triangle, /* ... */];
//! let kdtree = KDTree::build(&triangles);
//!
//! // Get a reduced list of triangles that a ray could intersect
//! let ray_origin = Vector3::zero();
//! let ray_direction = Vector3::new(1., 0., 0.);
//! let candidates_triangles = kdtree.intersect(&ray_origin, &ray_direction);
//! ```
mod aabb;
mod candidate;
mod kdnode;
mod kdtree;
mod plane;
mod ray;

pub use aabb::*;
pub use kdtree::KDTree;

type Point3 = cgmath::Vector3<f32>;
type Vector3 = cgmath::Vector3<f32>;

#[macro_use]
extern crate enum_map;
