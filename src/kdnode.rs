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
        let (cost, plane) = Self::partition(&items, &space);

        // Check that the cost of the splitting is not higher than the cost of the leaf.
        if cost > K_I * items.len() as f32 {
            return Self::Leaf {
                items: items.iter().cloned().collect(),
            };
        }

        // Compute the new spaces divided by `plane`
        let (left_space, right_space) = Self::split_space(&space, &plane);

        // Compute which items are part of the left and right space
        let (left_items, right_items) = Self::classify(&items, &left_space, &right_space);

        Self::Node {
            node: Box::new(InternalNode {
                left_node: Self::new(&left_space, left_items),
                right_node: Self::new(&right_space, right_items),
                left_space,
                right_space,
            }),
        }
    }

    /// Takes the items and space of a node and return the best splitting plane and his cost
    fn partition(items: &Items<P>, space: &AABB) -> (f32, Plane) {
        let (mut best_cost, mut best_plane) = (f32::INFINITY, Plane::X(0.));
        // For all the dimension
        for dim in 0..3 {
            for item in items {
                for plane in item.candidates(dim) {
                    // Compute the new spaces divided by `plane`
                    let (left_space, right_space) = Self::split_space(&space, &plane);

                    // Compute which items are part of the left and right space
                    let (left_items, right_items) =
                        Self::classify(&items, &left_space, &right_space);

                    // Compute the cost of the current plane
                    let cost = Self::cost(&plane, space, left_items.len(), right_items.len());

                    // If better update the best values
                    if cost < best_cost {
                        best_cost = cost;
                        best_plane = plane.clone();
                    }
                }
            }
        }
        (best_cost, best_plane)
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

    fn classify(items: &Items<P>, left_space: &AABB, right_space: &AABB) -> (Items<P>, Items<P>) {
        (
            // All items that overlap with the left space is taken
            items
                .iter()
                .filter(|item| left_space.intersect_box(&item.bb))
                .cloned()
                .collect(),
            // All items that overlap with the right space is taken
            items
                .iter()
                .filter(|item| right_space.intersect_box(&item.bb))
                .cloned()
                .collect(),
        )
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
