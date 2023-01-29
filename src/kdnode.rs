use std::collections::HashMap;

use crate::aabb::*;
use crate::candidate::{Candidates, Side};
use crate::config::BuilderConfig;
use crate::plane::{Dimension, Plane};

#[derive(Clone, Debug)]
pub enum KDTreeNode {
    Leaf {
        shapes: Vec<usize>,
    },
    Node {
        l_child: usize,
        l_space: AABB,
        r_child: usize,
        r_space: AABB,
    },
}

impl KDTreeNode {
    /// Move indices of the tree by `offset`.
    fn move_indices(&mut self, offset: usize) {
        match self {
            KDTreeNode::Leaf { .. } => {}
            KDTreeNode::Node {
                ref mut l_child,
                ref mut r_child,
                ..
            } => {
                *l_child += offset;
                *r_child += offset;
            }
        }
    }
}

/// Build a KDTree from a list of candidates and return the depth of the tree.
pub fn build_tree(
    config: &BuilderConfig,
    space: &AABB,
    candidates: Candidates,
    nb_shapes: usize,
) -> (usize, Vec<KDTreeNode>) {
    let (cost, best_index, n_l, n_r) = partition(config, nb_shapes, space, &candidates);

    // Check that the cost of the splitting is not higher than the cost of the leaf.
    if cost > config.cost_intersection() * nb_shapes as f32 {
        // Create indices values vector
        let shapes = candidates
            .iter()
            .filter(|e| e.is_left() && e.dimension() == Dimension::X)
            .map(|e| e.shape)
            .collect();
        return (1, vec![KDTreeNode::Leaf { shapes }]);
    }

    // Compute the new spaces divided by `plane`
    let (l_space, r_space) = split_space(space, &candidates[best_index].plane);

    // Compute which candidates are part of the left and right space
    let (left_candidates, right_candidates) = classify(candidates, best_index, nb_shapes);

    // Add left child
    let (left, right) = rayon::join(
        || build_tree(config, &l_space, left_candidates, n_l),
        || build_tree(config, &r_space, right_candidates, n_r),
    );

    let (depth_left, mut tree_left) = left;
    let (depth_right, mut tree_right) = right;
    let mut tree = vec![];

    // Add current node
    let l_child_index = 1;
    let r_child_index = tree_left.len() + 1;
    tree.push(KDTreeNode::Node {
        l_child: l_child_index,
        l_space,
        r_child: r_child_index,
        r_space,
    });

    // Update indices of the left tree.
    tree_left
        .iter_mut()
        .for_each(|node| node.move_indices(l_child_index));
    tree.extend(tree_left);

    // Update indices of the right tree.
    tree_right
        .iter_mut()
        .for_each(|node| node.move_indices(r_child_index));
    tree.extend(tree_right);

    (1 + depth_left.max(depth_right), tree)
}

/// Compute the best splitting candidate
/// Return:
/// * Cost of the split
/// * Index of the best candidate
/// * Number of items in the left partition
/// * Number of items in the right partition
fn partition(
    config: &BuilderConfig,
    n: usize,
    space: &AABB,
    candidates: &Candidates,
) -> (f32, usize, usize, usize) {
    let mut best_cost = f32::INFINITY;
    let mut best_candidate_index = 0;

    // Variables to keep count the number of items in both subspace for each dimension
    let mut n_l = Dimension::get_map(0);
    let mut n_r = Dimension::get_map(n);

    // Keep n_l and n_r for the best splitting candidate
    let mut best_n_l = 0;
    let mut best_n_r = n;

    // Find best candidate
    for (i, candidate) in candidates.iter().enumerate() {
        let dim = candidate.dimension();

        // If the right candidate removes it from the right subspace
        if candidate.is_right() {
            n_r[dim] -= 1;
        }

        // Compute the cost of the split and update the best split
        let cost = cost(config, &candidate.plane, space, n_l[dim], n_r[dim]);
        if cost < best_cost {
            best_cost = cost;
            best_candidate_index = i;
            best_n_l = n_l[dim];
            best_n_r = n_r[dim];
        }

        // If the left candidate add it from the left subspace
        if candidate.is_left() {
            n_l[dim] += 1;
        }
    }
    (best_cost, best_candidate_index, best_n_l, best_n_r)
}

/// Split an AABB in two subspaces given a splitting plane
fn split_space(space: &AABB, splitting_plane: &Plane) -> (AABB, AABB) {
    let mut left = space.clone();
    let mut right = space.clone();
    let pos = splitting_plane.pos;
    match splitting_plane.dimension {
        Dimension::X => {
            right.min.x = pos.clamp(space.min.x, space.max.x);
            left.max.x = pos.clamp(space.min.x, space.max.x);
        }
        Dimension::Y => {
            right.min.y = pos.clamp(space.min.y, space.max.y);
            left.max.y = pos.clamp(space.min.y, space.max.y);
        }
        Dimension::Z => {
            right.min.z = pos.clamp(space.min.z, space.max.z);
            left.max.z = pos.clamp(space.min.z, space.max.z);
        }
    }
    (left, right)
}

fn classify(
    candidates: Candidates,
    best_index: usize,
    nb_shapes: usize,
) -> (Candidates, Candidates) {
    let mut sides = HashMap::with_capacity(nb_shapes);
    // Step 1: Udate sides to classify items
    classify_items(&candidates, best_index, &mut sides);

    // Step 2: Splicing candidates left and right subspace
    splicing_candidates(candidates, &sides)
}

/// Step 1 of classify.
/// Given a candidate list and a splitting candidate identify wich items are part of the
/// left, right and both subspaces.
fn classify_items(candidates: &Candidates, best_index: usize, sides: &mut HashMap<usize, Side>) {
    let best_dimension = candidates[best_index].dimension();
    (0..(best_index + 1)).for_each(|i| {
        if candidates[i].dimension() == best_dimension {
            if candidates[i].is_right() {
                sides.insert(candidates[i].shape, Side::Left);
            } else {
                sides.insert(candidates[i].shape, Side::Both);
            }
        }
    });
    (best_index..candidates.len()).for_each(|i| {
        if candidates[i].dimension() == best_dimension && candidates[i].is_left() {
            sides.insert(candidates[i].shape, Side::Right);
        }
    });
}

// Step 2: Splicing candidates left and right subspace given items sides
fn splicing_candidates(
    mut candidates: Candidates,
    sides: &HashMap<usize, Side>,
) -> (Candidates, Candidates) {
    let mut left_candidates = Candidates::with_capacity(candidates.len() / 2);
    let mut right_candidates = Candidates::with_capacity(candidates.len() / 2);

    for e in candidates.drain(..) {
        match sides[&e.shape] {
            Side::Left => left_candidates.push(e),
            Side::Right => right_candidates.push(e),
            Side::Both => {
                right_candidates.push(e.clone());
                left_candidates.push(e);
            }
        }
    }
    (left_candidates, right_candidates)
}

/// Surface Area Heuristic (SAH)
fn cost(config: &BuilderConfig, plane: &Plane, space: &AABB, n_left: usize, n_right: usize) -> f32 {
    // If the plane doesn't cut the space, return max cost
    if !plane.is_cutting(space) {
        return f32::INFINITY;
    }

    // Compute the surface area of the whole space
    let surface_space = space.surface();

    // Split space
    let (space_left, space_right) = split_space(space, plane);

    // Compute the surface area of both subspace
    let surface_left = space_left.surface();
    let surface_right = space_right.surface();

    // Compute raw cost
    let cost = config.cost_traversal()
        + config.cost_intersection()
            * (n_left as f32 * surface_left / surface_space
                + n_right as f32 * surface_right / surface_space);

    // Decrease cost if it cuts empty space
    if n_left == 0 || n_right == 0 {
        cost * (1. - config.empty_cut_bonus())
    } else {
        cost
    }
}
