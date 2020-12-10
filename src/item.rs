use crate::plane::Plane;
use crate::{BoundingBox, AABB};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub type Items<P> = Vec<Arc<Item<P>>>;

#[derive(Debug)]
pub struct Item<P: BoundingBox> {
    pub value: Arc<P>,
    pub bb: AABB,
    pub id: usize,
}

impl<P: BoundingBox> Item<P> {
    pub fn new(value: P, bb: AABB, id: usize) -> Self {
        Item {
            value: Arc::new(value),
            bb,
            id,
        }
    }

    pub fn candidates(&self, dim: usize) -> Vec<Plane> {
        match dim {
            0 => {
                vec![Plane::X(self.bb.0.x), Plane::X(self.bb.1.x)]
            }
            1 => {
                vec![Plane::Y(self.bb.0.y), Plane::Y(self.bb.1.y)]
            }
            2 => {
                vec![Plane::Z(self.bb.0.z), Plane::Z(self.bb.1.z)]
            }
            _ => panic!("Invalid dimension number received: ({})", dim),
        }
    }
}

/// Implementation of the Clone will be needed when our item will have to
/// follow different branches of the tree.
impl<P: BoundingBox> Clone for Item<P> {
    fn clone(&self) -> Self {
        Item {
            value: self.value.clone(),
            bb: self.bb.clone(),
            id: self.id,
        }
    }
}

impl<P: BoundingBox> Hash for Item<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<P: BoundingBox> Eq for Item<P> {}
impl<P: BoundingBox> PartialEq for Item<P> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
