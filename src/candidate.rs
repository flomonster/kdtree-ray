use crate::plane::Plane;
use crate::AABB;
use std::cmp::Ordering;

pub type Candidates = Vec<Candidate>;

#[derive(Debug)]
pub struct Candidate {
    pub plane: Plane,
    pub is_left: bool,
    pub index: usize,
}

impl Candidate {
    fn new(plane: Plane, is_left: bool, index: usize) -> Self {
        Candidate {
            plane,
            is_left,
            index,
        }
    }

    /// Return candidates (splits candidates) for all dimension.
    pub fn gen_candidates(index: usize, bb: &AABB) -> Candidates {
        vec![
            Candidate::new(Plane::X(bb[0].x), true, index),
            Candidate::new(Plane::X(bb[1].x), false, index),
            Candidate::new(Plane::Y(bb[0].y), true, index),
            Candidate::new(Plane::Y(bb[1].y), false, index),
            Candidate::new(Plane::Z(bb[0].z), true, index),
            Candidate::new(Plane::Z(bb[1].z), false, index),
        ]
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

impl Clone for Candidate {
    fn clone(&self) -> Self {
        Self {
            plane: self.plane.clone(),
            is_left: self.is_left,
            index: self.index,
        }
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.plane.value() < other.plane.value() {
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
        self.plane.value() == other.plane.value() && self.dimension() == other.dimension()
    }
}

impl Eq for Candidate {}
