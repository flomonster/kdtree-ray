use crate::item::Item;
use crate::plane::Plane;
use crate::BoundingBox;
use std::cmp::Ordering;
use std::sync::Arc;

pub type Candidates<P> = Vec<Candidate<P>>;

#[derive(Debug)]
pub struct Candidate<P: BoundingBox> {
    pub plane: Plane,
    pub is_left: bool,
    pub item: Arc<Item<P>>,
}

impl<P: BoundingBox> Candidate<P> {
    fn new(plane: Plane, is_left: bool, item: Arc<Item<P>>) -> Self {
        Candidate {
            plane,
            is_left,
            item,
        }
    }

    /// Return candidates (splits candidates) for a given dimension.
    pub fn gen_candidates(item: Arc<Item<P>>, dim: usize) -> Candidates<P> {
        match dim {
            0 => vec![
                Candidate::new(Plane::X(item.bb.0.x), true, item.clone()),
                Candidate::new(Plane::X(item.bb.1.x), false, item),
            ],
            1 => vec![
                Candidate::new(Plane::Y(item.bb.0.y), true, item.clone()),
                Candidate::new(Plane::Y(item.bb.1.y), false, item),
            ],
            2 => vec![
                Candidate::new(Plane::Z(item.bb.0.z), true, item.clone()),
                Candidate::new(Plane::Z(item.bb.1.z), false, item),
            ],
            _ => panic!("Invalid dimension number received: ({})", dim),
        }
    }

    pub fn dimension(&self) -> usize {
        match self.plane {
            Plane::X(_) => 0,
            Plane::Y(_) => 1,
            Plane::Z(_) => 2,
        }
    }

    pub fn is_left(&self) -> bool {
        self.is_left
    }

    pub fn is_right(&self) -> bool {
        !self.is_left
    }
}

impl<P: BoundingBox> Clone for Candidate<P> {
    fn clone(&self) -> Self {
        Self {
            plane: self.plane.clone(),
            is_left: self.is_left,
            item: self.item.clone(),
        }
    }
}

impl<P: BoundingBox> Ord for Candidate<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.plane.value() < other.plane.value() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
impl<P: BoundingBox> PartialOrd for Candidate<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: BoundingBox> PartialEq for Candidate<P> {
    fn eq(&self, other: &Self) -> bool {
        self.plane.value() == other.plane.value() && self.dimension() == other.dimension()
    }
}

impl<P: BoundingBox> Eq for Candidate<P> {}
