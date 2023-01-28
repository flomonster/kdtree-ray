static DEFAULT_COST_TRAVERSAL: f32 = 15.;
static DEFAULT_COST_INTERSECTION: f32 = 20.;
static DEFAULT_EMPTY_CUT_BONUS: f32 = 0.2;

/// Configuration for the builder.
#[derive(Clone, Copy, Debug)]
pub struct BuilderConfig {
    /// Cost of a traversal in the kdtree.
    cost_traversal: f32,
    /// Cost of an intersection test.
    cost_intersection: f32,
    /// Bonus (between `0.` and `1.`) for cutting an empty space:
    /// * `1.` means that cutting an empty space is in any case better than cutting a full space.
    /// * `0.` means that cutting an empty space isn't better than cutting a full space.
    empty_cut_bonus: f32,
}

impl BuilderConfig {
    /// Create a new `BuilderConfig` given the cost of a traversal, the cost of an intersection
    /// test and the bonus for cutting an empty space.
    ///
    /// ### Panics
    ///
    /// * If `cost_traversal` is not strictly positive.
    /// * If `cost_intersection` is not strictly positive.
    /// * If `empty_cut_bonus` is not between `0.` and `1.`.
    pub fn new(cost_traversal: f32, cost_intersection: f32, empty_cut_bonus: f32) -> Self {
        assert!(cost_traversal > 0.);
        assert!(cost_intersection > 0.);
        assert!((0. ..=1.).contains(&empty_cut_bonus));
        BuilderConfig {
            cost_traversal,
            cost_intersection,
            empty_cut_bonus,
        }
    }

    /// Retrieve the cost of a traversal.
    pub fn cost_traversal(&self) -> f32 {
        self.cost_traversal
    }

    /// Retrieve the cost of an intersection.
    pub fn cost_intersection(&self) -> f32 {
        self.cost_intersection
    }

    /// Retrieve the bonus for cutting an empty space.
    pub fn empty_cut_bonus(&self) -> f32 {
        self.empty_cut_bonus
    }
}

impl Default for BuilderConfig {
    /// Create a new `BuilderConfig` with the default values.
    /// * Traversal cost: `15.`
    /// * Intersection cost: `20.`
    /// * Empty cut bonus: `0.2`
    fn default() -> Self {
        BuilderConfig {
            cost_traversal: DEFAULT_COST_TRAVERSAL,
            cost_intersection: DEFAULT_COST_INTERSECTION,
            empty_cut_bonus: DEFAULT_EMPTY_CUT_BONUS,
        }
    }
}
