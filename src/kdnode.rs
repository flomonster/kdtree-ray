use crate::candidate::{Candidate, Candidates};
use crate::plane::Plane;
use crate::*;
use cgmath::*;
use std::collections::HashSet;
use std::sync::Arc;

static K_T: f32 = 15.;
static K_I: f32 = 20.;

#[derive(Clone, Debug)]
pub struct InternalNode<P: BoundingBox> {
    left_space: AABB,
    left_node: KDtreeNode<P>,
    right_space: AABB,
    right_node: KDtreeNode<P>,
}

#[derive(Clone, Debug)]
pub enum KDtreeNode<P: BoundingBox> {
    Leaf { items: HashSet<Arc<Item<P>>> },
    Node { node: Box<InternalNode<P>> },
}

impl<P: BoundingBox> KDtreeNode<P> {
    pub fn new(space: &AABB, items: Items<P>) -> Self {
        let (cost, candidates, best_index) = Self::partition(&items, &space);

        // Check that the cost of the splitting is not higher than the cost of the leaf.
        if cost > K_I * items.len() as f32 {
            return Self::Leaf {
                items: items.iter().cloned().collect(),
            };
        }

        // Compute the new spaces divided by `plane`
        let (left_space, right_space) = Self::split_space(&space, &candidates[best_index].plane);

        // Compute which items are part of the left and right space
        let (left_items, right_items) = Self::classify(&candidates, best_index);

        Self::Node {
            node: Box::new(InternalNode {
                left_node: Self::new(&left_space, left_items),
                right_node: Self::new(&right_space, right_items),
                left_space,
                right_space,
            }),
        }
    }

    /// Compute the best splitting candidate
    /// Return:
    /// * Cost of the split
    /// * The list of candidates (in the best dimension found)
    /// * Index of the best candidate
    fn partition(items: &Items<P>, space: &AABB) -> (f32, Candidates<P>, usize) {
        let mut best_cost = f32::INFINITY;
        let mut best_candidate_index = 0;
        let mut best_candidates = vec![];

        // For all the dimension
        for dim in 0..3 {
            // Generate candidates
            let mut candidates = vec![];
            for item in items {
                candidates.append(&mut Candidate::gen_candidates(item.clone(), dim));
            }
            // Sort candidates
            candidates.sort_by(|a, b| a.cmp(&b));

            let mut n_r = items.len();
            let mut n_l = 0;
            let mut best_dim = false;

            // Find best candidate
            for (i, candidate) in candidates.iter().enumerate() {
                if candidate.is_right() {
                    n_r -= 1;
                }

                // Compute the cost of the current plane
                let cost = Self::cost(&candidate.plane, space, n_l, n_r);

                // If better update the best values
                if cost < best_cost {
                    best_cost = cost;
                    best_candidate_index = i;
                    best_dim = true;
                }

                if candidate.is_left() {
                    n_l += 1;
                }
            }
            if best_dim {
                best_candidates = candidates;
            }
        }
        (best_cost, best_candidates, best_candidate_index)
    }

    pub fn intersect(
        &self,
        ray_origin: &Vector3<f32>,
        ray_direction: &Vector3<f32>,
        intersect_items: &mut HashSet<Arc<Item<P>>>,
    ) {
        match self {
            Self::Leaf { items } => {
                intersect_items.extend(items.clone());
            }
            Self::Node { node } => {
                if node.right_space.intersect_ray(ray_origin, ray_direction) {
                    node.right_node
                        .intersect(ray_origin, ray_direction, intersect_items);
                }
                if node.left_space.intersect_ray(ray_origin, ray_direction) {
                    node.left_node
                        .intersect(ray_origin, ray_direction, intersect_items);
                }
            }
        }
    }

    fn split_space(space: &AABB, plane: &Plane) -> (AABB, AABB) {
        let mut left = space.clone();
        let mut right = space.clone();
        match plane {
            Plane::X(x) => {
                left.1.x = x.max(space.0.x).min(space.1.x);
                right.0.x = x.max(space.0.x).min(space.1.x);
            }
            Plane::Y(y) => {
                left.1.y = y.max(space.0.y).min(space.1.y);
                right.0.y = y.max(space.0.y).min(space.1.y);
            }
            Plane::Z(z) => {
                left.1.z = z.max(space.0.z).min(space.1.z);
                right.0.z = z.max(space.0.z).min(space.1.z);
            }
        }
        (left, right)
    }

    fn classify(candidates: &Candidates<P>, best_index: usize) -> (Items<P>, Items<P>) {
        let mut left_items = Items::with_capacity(candidates.len() / 3);
        let mut right_items = Items::with_capacity(candidates.len() / 3);

        for i in 0..best_index {
            if candidates[i].is_left() {
                left_items.push(candidates[i].item.clone());
            }
        }
        for i in (1 + best_index)..candidates.len() {
            if candidates[i].is_right() {
                right_items.push(candidates[i].item.clone());
            }
        }
        (left_items, right_items)
    }

    /// Compute surface area volume of a space (AABB).
    fn surface_area(v: &AABB) -> f32 {
        (v.1.x - v.0.x) * (v.1.y - v.0.y) * (v.1.z - v.0.z)
    }

    /// Surface Area Heuristic (SAH)
    fn cost(p: &Plane, v: &AABB, n_l: usize, n_r: usize) -> f32 {
        // Split space
        let (v_l, v_r) = Self::split_space(v, p);

        // Compute the surface area of both subspace
        let (vol_l, vol_r) = (Self::surface_area(&v_l), Self::surface_area(&v_r));

        // Compute the surface area of the whole space
        let vol_v = vol_l + vol_r;

        // If one of the subspace is empty then the split can't be worth
        if vol_v == 0. || vol_l == 0. || vol_r == 0. {
            return f32::INFINITY;
        }

        // Decrease cost if it cuts empty space
        let factor = if n_l == 0 || n_r == 0 { 0.8 } else { 1. };

        factor * (K_T + K_I * (n_l as f32 * vol_l / vol_v + n_r as f32 * vol_r / vol_v))
    }
}
