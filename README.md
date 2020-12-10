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

This crate is a fast implementation of [KD-tree](https://en.wikipedia.org/wiki/K-d_tree)
for raytracer (or other rendering method using ray).

It's based on this [paper](http://www.irisa.fr/prive/kadi/Sujets_CTR/kadi/Kadi_sujet2_article_Kdtree.pdf) written by *Ingo Wald* and *Vlastimil Havran*.

### Installation

To install it, just add the dependency in your `Cargo.toml`.

```toml
[dependencies]
kdtree-ray="0.1.0"
```

### Usage

For examples of use see the [crate documentation](https://docs.rs/kdtree-ray).
