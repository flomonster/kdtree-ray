use crate::candidate::Candidates;
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
    pub fn new(
        space: &AABB,
        mut candidates: Candidates<P>,
        n: usize,
        sides: &mut Vec<Side>,
    ) -> Self {
        let (cost, best_index, n_l, n_r) = Self::partition(n, &space, &candidates);

        // Check that the cost of the splitting is not higher than the cost of the leaf.
        if cost > K_I * n as f32 {
            // Create the set of primitives
            let mut items = HashSet::with_capacity(n);
            candidates
                .drain(..)
                .filter(|e| e.is_left() && e.dimension() == 0)
                .for_each(|e| {
                    items.insert(e.item);
                });
            return Self::Leaf { items };
        }

        // Compute the new spaces divided by `plane`
        let (left_space, right_space) = Self::split_space(&space, &candidates[best_index].plane);

        // Compute which candidates are part of the left and right space
        let (left_candidates, right_candidates) = Self::classify(candidates, best_index, sides);

        Self::Node {
            node: Box::new(InternalNode {
                left_node: Self::new(&left_space, left_candidates, n_l, sides),
                right_node: Self::new(&right_space, right_candidates, n_r, sides),
                left_space,
                right_space,
            }),
        }
    }

    /// Compute the best splitting candidate
    /// Return:
    /// * Cost of the split
    /// * Index of the best candidate
    /// * Number of items in the left partition
    /// * Number of items in the right partition
    fn partition(n: usize, space: &AABB, candidates: &Candidates<P>) -> (f32, usize, usize, usize) {
        let mut best_cost = f32::INFINITY;
        let mut best_candidate_index = 0;

        // Variables to keep count the number of items in both subspace for each dimension
        let mut n_l = [0; 3];
        let mut n_r = [n; 3];

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
            let cost = Self::cost(&candidate.plane, space, n_l[dim], n_r[dim]);
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

    fn classify(
        candidates: Candidates<P>,
        best_index: usize,
        sides: &mut Vec<Side>,
    ) -> (Candidates<P>, Candidates<P>) {
        // Step 1: Udate sides to classify items
        Self::classify_items(&candidates, best_index, sides);

        // Step 2: Splicing candidates left and right subspace
        Self::splicing_candidates(candidates, &sides)
    }

    /// Step 1 of classify.
    /// Given a candidate list and a splitting candidate identify wich items are part of the
    /// left, right and both subspaces.
    fn classify_items(candidates: &Candidates<P>, best_index: usize, sides: &mut Vec<Side>) {
        let best_dimension = candidates[best_index].dimension();
        for i in 0..(best_index + 1) {
            if candidates[i].dimension() == best_dimension {
                if candidates[i].is_right() {
                    sides[candidates[i].item.id] = Side::Left;
                } else {
                    sides[candidates[i].item.id] = Side::Both;
                }
            }
        }
        for i in best_index..candidates.len() {
            if candidates[i].dimension() == best_dimension && candidates[i].is_left() {
                sides[candidates[i].item.id] = Side::Right;
            }
        }
    }

    // Step 2: Splicing candidates left and right subspace given items sides
    fn splicing_candidates(
        mut candidates: Candidates<P>,
        sides: &Vec<Side>,
    ) -> (Candidates<P>, Candidates<P>) {
        let mut left_candidates = Candidates::with_capacity(candidates.len() / 2);
        let mut right_candidates = Candidates::with_capacity(candidates.len() / 2);

        for e in candidates.drain(..) {
            match sides[e.item.id] {
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
