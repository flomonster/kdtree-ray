use crate::plane::Plane;
use crate::*;
use cgmath::*;
use std::collections::HashSet;
use std::sync::Arc;

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
    pub fn new(space: &AABB, items: Items<P>, max_depth: usize) -> Self {
        // Heuristic to terminate the recursion
        if items.len() <= 15 || max_depth == 0 {
            return Self::Leaf {
                items: items.iter().cloned().collect(),
            };
        }

        // Find a plane to partition the space
        let plane = Self::partition(&space, max_depth);

        // Compute the new spaces divided by `plane`
        let (left_space, right_space) = Self::split_space(&space, &plane);

        // Compute which items are part of the left and right space
        let (left_items, right_items) = Self::classify(&items, &left_space, &right_space);

        Self::Node {
            node: Box::new(InternalNode {
                left_node: Self::new(&left_space, left_items, max_depth - 1),
                right_node: Self::new(&right_space, right_items, max_depth - 1),
                left_space,
                right_space,
            }),
        }
    }

    fn partition(space: &AABB, max_depth: usize) -> Plane {
        match max_depth % 3 {
            0 => Plane::X((space.0.x + space.1.x) / 2.),
            1 => Plane::Y((space.0.y + space.1.y) / 2.),
            _ => Plane::Z((space.0.z + space.1.z) / 2.),
        }
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
}
