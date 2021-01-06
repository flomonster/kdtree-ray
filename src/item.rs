use crate::BoundingBox;
use std::sync::Arc;

#[derive(Debug)]
pub struct Item<P: BoundingBox> {
    pub value: Arc<P>,
    pub id: usize,
}

impl<P: BoundingBox> Item<P> {
    pub fn new(value: P, id: usize) -> Self {
        Item {
            value: Arc::new(value),
            id,
        }
    }
}

/// Implementation of the Clone will be needed when our item will have to
/// follow different branches of the tree.
impl<P: BoundingBox> Clone for Item<P> {
    fn clone(&self) -> Self {
        Item {
            value: self.value.clone(),
            id: self.id,
        }
    }
}
