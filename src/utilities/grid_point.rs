pub struct GridPoint {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl PartialEq for GridPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
