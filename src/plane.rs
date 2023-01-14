use enum_map::{Enum, EnumMap};

/// 3D dimensions enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Enum)]
pub enum Dimension {
    X,
    Y,
    Z,
}

impl Dimension {
    pub fn get_map<T: Clone>(value: T) -> EnumMap<Dimension, T> {
        enum_map! {
            Dimension::X => value.clone(),
            Dimension::Y => value.clone(),
            Dimension::Z => value.clone()
        }
    }
}

/// 3D plane.
#[derive(Clone, Debug)]
pub struct Plane {
    pub dimension: Dimension,
    pub pos: f32,
}

impl Plane {
    /// Create a new plane.
    pub fn new(dimension: Dimension, pos: f32) -> Self {
        Plane { dimension, pos }
    }

    /// Create a new plane on the X axis.
    pub fn new_x(pos: f32) -> Self {
        Plane::new(Dimension::X, pos)
    }

    /// Create a new plane on the Y axis.
    pub fn new_y(pos: f32) -> Self {
        Plane::new(Dimension::Y, pos)
    }

    /// Create a new plane on the Z axis.
    pub fn new_z(pos: f32) -> Self {
        Plane::new(Dimension::Z, pos)
    }
}
