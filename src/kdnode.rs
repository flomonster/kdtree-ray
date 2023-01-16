use crate::aabb::*;
use crate::candidate::{Candidates, Side};
use crate::plane::{Dimension, Plane};

static COST_TRAVERSAL: f32 = 4.;
static COST_INTERSECTION: f32 = 1.;

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
    pub fn set_r_child_index(&mut self, index: usize) {
        if let KDTreeNode::Node {
            ref mut r_child, ..
        } = self
        {
            *r_child = index;
        } else {
            panic!("Cannot set r_child index on a leaf node");
        }
    }
}

/// Build a KDTree from a list of candidates and return the depth of the tree.
pub fn build_tree(
    space: &AABB,
    candidates: Candidates,
    nb_shapes: usize,
    sides: &mut [Side],
    tree: &mut Vec<KDTreeNode>,
) -> usize {
    let (cost, best_index, n_l, n_r) = partition(nb_shapes, space, &candidates);

    // Check that the cost of the splitting is not higher than the cost of the leaf.
    if cost > COST_INTERSECTION * nb_shapes as f32 {
        // Create indices values vector
        let shapes = candidates
            .iter()
            .filter(|e| e.is_left() && e.dimension() == Dimension::X)
            .map(|e| e.shape)
            .collect();
        tree.push(KDTreeNode::Leaf { shapes });
        return 1;
    }

    // Compute the new spaces divided by `plane`
    let (left_space, right_space) = split_space(space, &candidates[best_index].plane);

    // Compute which candidates are part of the left and right space
    let (left_candidates, right_candidates) = classify(candidates, best_index, sides);

    // Add current node
    let node_index = tree.len();
    tree.push(KDTreeNode::Node {
        l_child: node_index + 1,
        l_space: left_space.clone(),
        r_child: 0, // Filled later
        r_space: right_space.clone(),
    });

    // Add left child
    let depth_left = build_tree(&left_space, left_candidates, n_l, sides, tree);

    let r_child_index = tree.len();
    tree[node_index].set_r_child_index(r_child_index);

    // Add right
    let depth_right = build_tree(&right_space, right_candidates, n_r, sides, tree);

    1 + depth_left.max(depth_right)
}

/// Compute the best splitting candidate
/// Return:
/// * Cost of the split
/// * Index of the best candidate
/// * Number of items in the left partition
/// * Number of items in the right partition
fn partition(n: usize, space: &AABB, candidates: &Candidates) -> (f32, usize, usize, usize) {
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
        let cost = cost(&candidate.plane, space, n_l[dim], n_r[dim]);
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
    sides: &mut [Side],
) -> (Candidates, Candidates) {
    // Step 1: Udate sides to classify items
    classify_items(&candidates, best_index, sides);

    // Step 2: Splicing candidates left and right subspace
    splicing_candidates(candidates, sides)
}

/// Step 1 of classify.
/// Given a candidate list and a splitting candidate identify wich items are part of the
/// left, right and both subspaces.
fn classify_items(candidates: &Candidates, best_index: usize, sides: &mut [Side]) {
    let best_dimension = candidates[best_index].dimension();
    for i in 0..(best_index + 1) {
        if candidates[i].dimension() == best_dimension {
            if candidates[i].is_right() {
                sides[candidates[i].shape] = Side::Left;
            } else {
                sides[candidates[i].shape] = Side::Both;
            }
        }
    }
    for i in best_index..candidates.len() {
        if candidates[i].dimension() == best_dimension && candidates[i].is_left() {
            sides[candidates[i].shape] = Side::Right;
        }
    }
}

// Step 2: Splicing candidates left and right subspace given items sides
fn splicing_candidates(mut candidates: Candidates, sides: &[Side]) -> (Candidates, Candidates) {
    let mut left_candidates = Candidates::with_capacity(candidates.len() / 2);
    let mut right_candidates = Candidates::with_capacity(candidates.len() / 2);

    for e in candidates.drain(..) {
        match sides[e.shape] {
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
fn cost(plane: &Plane, space: &AABB, n_left: usize, n_right: usize) -> f32 {
    // Split space
    let (space_left, space_right) = split_space(space, plane);

    // Compute the surface area of both subspace
    let vol_left = space_left.volume();
    let vol_right = space_right.volume();

    // Compute the surface area of the whole space
    let vol_space = vol_left + vol_right;

    // If one of the subspace is empty then the split can't be worth
    if vol_space == 0. || vol_left == 0. || vol_right == 0. {
        return f32::INFINITY;
    }

    // Compute raw cost
    let cost = COST_TRAVERSAL
        + COST_INTERSECTION
            * (n_left as f32 * vol_left / vol_space + n_right as f32 * vol_right / vol_space);

    // Decrease cost if it cuts empty space
    if n_left == 0 || n_right == 0 {
        cost * 0.8
    } else {
        cost
    }
}
