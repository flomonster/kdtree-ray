use crate::aabb::*;
use crate::candidate::*;
use crate::config::BuilderConfig;
use crate::kdnode::{build_tree, KDTreeNode};
use crate::ray::Ray;
use crate::Vector3;

/// The KD-tree data structure.
#[derive(Clone, Debug)]
pub struct KDTree {
    tree: Vec<KDTreeNode>,
    space: AABB,
    depth: usize,
}

impl KDTree {
    /// This function is used to build a new KD-tree. You need to provide a
    /// `Vec` of shapes that implement `Bounded` trait.
    /// You also should give a configuration.
    /// Panic if the `shapes` is empty.
    pub fn build_config<S: Bounded>(shapes: &Vec<S>, config: &BuilderConfig) -> Self {
        assert!(!shapes.is_empty());
        let mut space = AABB::default();
        let mut candidates = Candidates::with_capacity(shapes.len() * 6);
        for (index, v) in shapes.iter().enumerate() {
            // Create items from values
            let bb = v.bound();
            candidates.extend(Candidate::gen_candidates(index, &bb));

            // Update space with the bounding box of the item
            space.merge(&bb);
        }

        // Sort candidates only once at the begining
        candidates.sort();

        // Will be used to classify candidates
        let mut sides = vec![Side::Both; shapes.len()];

        let nb_shapes = shapes.len();

        // Build the tree
        let mut tree = vec![];
        let depth = build_tree(config, &space, candidates, nb_shapes, &mut sides, &mut tree);

        KDTree { space, tree, depth }
    }

    /// This function is used to build a new KD-tree. You need to provide a
    /// `Vec` of shapes that implement `Bounded` trait.
    /// Take a default configuration.
    /// Panic if the `shapes` is empty.
    pub fn build<S: Bounded>(shapes: &Vec<S>) -> Self {
        Self::build_config(shapes, &BuilderConfig::default())
    }

    /// This function takes a ray and return a reduced list of shapes that
    /// can be intersected by the ray.
    pub fn intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Vec<usize> {
        let ray = Ray::new(ray_origin, ray_direction);
        let mut result = vec![];
        let mut stack = vec![0];
        stack.reserve_exact(self.depth);
        while !stack.is_empty() {
            let node = &self.tree[stack.pop().unwrap()];
            match node {
                KDTreeNode::Leaf { shapes } => result.extend(shapes),
                KDTreeNode::Node {
                    l_child,
                    l_space,
                    r_child,
                    r_space,
                } => {
                    if ray.intersect(r_space) {
                        stack.push(*r_child)
                    }
                    if ray.intersect(l_space) {
                        stack.push(*l_child)
                    }
                }
            }
        }
        // Dedup duplicated shapes
        result.sort();
        result.dedup();
        result
    }
}

impl Bounded for KDTree {
    fn bound(&self) -> AABB {
        self.space.clone()
    }
}
