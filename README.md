<h1 align="center">
    kdtree-ray
</h1>
<p align="center">
   <a href="https://github.com/flomonster/kdtree-ray/actions">
      <img src="https://github.com/flomonster/kdtree-ray/workflows/Build/badge.svg" alt="github">
   </a>
   <a href="https://crates.io/crates/kdtree-ray">
      <img src="https://img.shields.io/crates/v/kdtree-ray.svg" alt="crates.io">
   </a>
   <a href="https://docs.rs/kdtree-ray">
      <img src="https://docs.rs/kdtree-ray/badge.svg" alt="docs.rs">
   </a>
</p>
<hr>

This crate is a fast implementation of [KD-Tree](https://en.wikipedia.org/wiki/K-d_tree)
for raytracer (or other rendering method using ray).

It's based on this [paper](http://www.irisa.fr/prive/kadi/Sujets_CTR/kadi/Kadi_sujet2_article_Kdtree.pdf) written by *Ingo Wald* and *Vlastimil Havran*.

For more information on how this library is implemented check out [my article](https://www.flomonster.fr/articles/kdtree.html).

### Installation

To install it, just add the dependency in your `Cargo.toml`.

```toml
[dependencies]
kdtree-ray="1.2.2"
```

### Usage

```rust
use cgmath::*;
use kdtree_ray::{AABB, Bounded, KDTree};

struct Triangle(Vector3<f32>, Vector3<f32>, Vector3<f32>);

// To use the KDTree on an object you need first to implement the BoundingBox trait.
impl Bounded for Triangle {
  fn bound(&self) -> AABB {
    let min = Vector3::new(
      self.0.x.min(self.1.x).min(self.2.x),
      self.0.y.min(self.1.y).min(self.2.y),
      self.0.z.min(self.1.z).min(self.2.z),
    );
    let max = Vector3::new(
      self.0.x.max(self.1.x).max(self.2.x),
      self.0.y.max(self.1.y).max(self.2.y),
      self.0.z.max(self.1.z).max(self.2.z),
    );
    AABB::new(min, max)
  }
}

// Kdtree creation
let triangle = Triangle(Vector3::zero(), Vector3::zero(), Vector3::zero());
let triangles: Vec<Triangle> = vec![triangle, /* ... */];
let kdtree = KDTree::build(&triangles);

// Get a reduced list of triangles that a ray could intersect
let ray_origin = Vector3::zero();
let ray_direction = Vector3::new(1., 0., 0.);
let candidates_triangles = kdtree.intersect(&ray_origin, &ray_direction);
```

Examples of projects using this crate:

- [Path Tracer](https://github.com/flomonster/path-tracer)
