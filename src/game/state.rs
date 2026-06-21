#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Easy,
    Medium,
    Hard,
    Extreme,
}

impl Mode {
    pub fn prefilled_count(self) -> usize {
        match self {
            Mode::Easy => 3,
            Mode::Medium => 2,
            Mode::Hard => 1,
            Mode::Extreme => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Grid([[Option<u8>; 2]; 2]);

impl Grid {
    pub fn new() -> Self {
        Grid([[None; 2]; 2])
    }

    pub fn cells(&self) -> &[[Option<u8>; 2]; 2] {
        &self.0
    }

    pub fn get(&self, row: usize, col: usize) -> Option<u8> {
        self.0[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: Option<u8>) {
        self.0[row][col] = value;
    }

    pub fn is_complete(&self) -> bool {
        self.0.iter().all(|row| row.iter().all(|c| c.is_some()))
    }

    pub fn filled_values(&self) -> Vec<u8> {
        self.0.iter().flatten().filter_map(|&c| c).collect()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl From<[[u8; 2]; 2]> for Grid {
    fn from(arr: [[u8; 2]; 2]) -> Self {
        Grid(arr.map(|row| row.map(Some)))
    }
}

impl From<Grid> for [[Option<u8>; 2]; 2] {
    fn from(grid: Grid) -> Self {
        grid.0
    }
}
