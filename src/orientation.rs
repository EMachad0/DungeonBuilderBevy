#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    pub fn opposite(&self) -> Self {
        match self {
            Orientation::North => Orientation::South,
            Orientation::East => Orientation::West,
            Orientation::South => Orientation::North,
            Orientation::West => Orientation::East,
        }
    }
}
