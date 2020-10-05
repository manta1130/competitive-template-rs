use std::cmp::Ordering;
use std::ops::Deref;

#[derive(PartialEq)]
pub struct OrdFloat(pub f64);

impl Ord for OrdFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for OrdFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Deref for OrdFloat {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl Eq for OrdFloat {}
