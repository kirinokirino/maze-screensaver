#[derive(Clone, Copy, Debug)]
struct GridPosition {
    x: i32,
    y: i32,
}

impl GridPosition {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn get_neighbours(&self) -> [GridPosition; 4] {
        [
            GridPosition::new(self.x - 1, self.y),
            GridPosition::new(self.x + 1, self.y),
            GridPosition::new(self.x, self.y - 1),
            GridPosition::new(self.x, self.y + 1),
        ]
    }
}

pub struct Grid {
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    fn is_outside(&self, pos: GridPosition) -> bool {
        pos.x < 0 && pos.y < 0 && pos.x >= self.width as i32 && pos.y >= self.height as i32
    }
}
