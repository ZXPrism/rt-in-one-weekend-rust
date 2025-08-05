#[derive(Clone, Copy)]
pub struct Interval {
    pub left: f64,
    pub right: f64,
}

impl Interval {
    pub fn new(left: f64, right: f64) -> Interval {
        Interval { left, right }
    }

    pub fn contains(&self, test_value: f64) -> bool {
        self.left <= test_value && test_value <= self.right
    }

    pub fn overlap(&self, rhs: Interval) -> bool {
        self.right >= rhs.left && self.left <= rhs.right
    }
}
