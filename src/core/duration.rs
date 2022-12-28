#[derive(Debug, Clone, PartialEq)]
pub struct Duration(pub chrono::Duration);

impl Duration {
    pub fn zero() -> Self {
        Self(chrono::Duration::zero())
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self::zero()
    }
}
