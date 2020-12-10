#[derive(Clone, Debug)]
pub enum Plane {
    X(f32),
    Y(f32),
    Z(f32),
}

impl Plane {
    pub fn value(&self) -> f32 {
        match self {
            Plane::X(v) => *v,
            Plane::Y(v) => *v,
            Plane::Z(v) => *v,
        }
    }
}
