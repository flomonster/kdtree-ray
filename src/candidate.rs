use crate::AABB;
use crate::plane::{Dimension, Plane};
use std::cmp::Ordering;

pub type Candidates = Vec<Candidate>;

#[derive(Debug, Clone)]
pub struct Candidate {
    pub plane: Plane,
    pub is_left: bool,
    pub shape: usize,
}

impl Candidate {
    fn new(plane: Plane, is_left: bool, index: usize) -> Self {
        Candidate {
            plane,
            is_left,
            shape: index,
        }
    }

    /// Return candidates (splits candidates) for all dimension.
    pub fn gen_candidates(shape: usize, bb: &AABB) -> Candidates {
        vec![
            Candidate::new(Plane::new_x(bb.min.x), true, shape),
            Candidate::new(Plane::new_x(bb.max.x), false, shape),
            Candidate::new(Plane::new_y(bb.min.y), true, shape),
            Candidate::new(Plane::new_y(bb.max.y), false, shape),
            Candidate::new(Plane::new_z(bb.min.z), true, shape),
            Candidate::new(Plane::new_z(bb.max.z), false, shape),
        ]
    }

    pub fn dimension(&self) -> Dimension {
        self.plane.dimension
    }

    pub fn position(&self) -> f32 {
        self.plane.pos
    }

    pub fn is_left(&self) -> bool {
        self.is_left
    }

    pub fn is_right(&self) -> bool {
        !self.is_left
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.position() < other.position() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.position() == other.position() && self.dimension() == other.dimension()
    }
}

impl Eq for Candidate {}

/// Useful to classify candidates
#[derive(Debug, Clone, Copy)]
pub enum Side {
    Left,
    Right,
    Both,
}
