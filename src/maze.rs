use speedy2d::{color::Color, dimen::Vector2, shape::Rectangle, Graphics2D};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GridPosition {
    x: i32,
    y: i32,
}

impl GridPosition {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn random(max_width: usize, max_height: usize) -> Self {
        Self {
            x: fastrand::i32(0..max_width as i32),
            y: fastrand::i32(0..max_height as i32),
        }
    }

    pub fn get_neighbours(&self) -> [GridPosition; 4] {
        [
            GridPosition::new(self.x - 1, self.y),
            GridPosition::new(self.x + 1, self.y),
            GridPosition::new(self.x, self.y - 1),
            GridPosition::new(self.x, self.y + 1),
        ]
    }

    pub fn as_vector2(&self, scale: f32) -> Vector2<f32> {
        Vector2::new(self.x as f32 * scale, self.y as f32 * scale)
    }
}

#[derive(Debug, Clone)]
struct Walker {
    path: Vec<GridPosition>,
}

impl Walker {
    fn new(starting_position: GridPosition) -> Self {
        Self {
            path: vec![starting_position],
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D, scale: f32) {
        let half_cell_size = Vector2::new(scale as f32 / 2.0, scale as f32 / 2.0);

        for path_segment in self.path.windows(2) {
            graphics.draw_line(
                path_segment[0].as_vector2(scale) + half_cell_size,
                path_segment[1].as_vector2(scale) + half_cell_size,
                2.0,
                Color::BLACK,
            );
        }

        if let Some(last_position) = self.path.last() {
            let last_position = last_position.as_vector2(scale);
            let padding = half_cell_size / 2.0;
            graphics.draw_rectangle(
                Rectangle::new(
                    last_position + padding,
                    last_position + half_cell_size + padding,
                ),
                Color::from_rgb(0.7, 0.3, 0.7),
            );
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MazeState {
    Generating,
    Finished,
}

pub struct Maze {
    scale: f32,
    width: usize,
    height: usize,

    start: GridPosition,
    finish: GridPosition,

    walker: Walker,
    visited_cells: Vec<GridPosition>,
    backtrack: Vec<GridPosition>,
    tracks: Vec<Vec<GridPosition>>,

    state: MazeState,
}

impl Maze {
    pub fn new(width: usize, height: usize, scale: f32) -> Self {
        let start = GridPosition::random(width, height);
        let mut finish = GridPosition::random(width, height);
        while finish == start {
            finish = GridPosition::random(width, height);
        }
        let finish = finish;

        Self {
            scale,
            width,
            height,
            start,
            finish,
            walker: Walker::new(start),
            visited_cells: vec![start],
            backtrack: Vec::new(),
            tracks: Vec::new(),
            state: MazeState::Generating,
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        let cell_size = Vector2::new(self.scale as f32, self.scale as f32);
        let half_cell_size = cell_size / 2.0;

        let padding = Vector2::new(0.1, 0.1);
        for cell in self.visited_cells.iter() {
            let pos = cell.as_vector2(self.scale);
            graphics.draw_rectangle(
                Rectangle::new(pos + padding, pos + cell_size - padding - padding),
                Color::from_gray(0.8),
            )
        }

        for track in self.tracks.iter() {
            for path_segment in track.windows(2) {
                graphics.draw_line(
                    path_segment[0].as_vector2(self.scale) + half_cell_size,
                    path_segment[1].as_vector2(self.scale) + half_cell_size,
                    2.0,
                    Color::from_gray(0.3),
                );
            }
        }

        let start_pos = self.start.as_vector2(self.scale);
        graphics.draw_rectangle(
            Rectangle::new(start_pos, start_pos + cell_size),
            Color::from_rgb(0.2, 0.6, 0.3),
        );

        let finish_pos = self.finish.as_vector2(self.scale);
        graphics.draw_rectangle(
            Rectangle::new(finish_pos, finish_pos + cell_size),
            Color::from_rgb(0.2, 0.3, 0.7),
        );

        self.walker.draw(graphics, self.scale);
    }

    pub fn step(&mut self) {
        if self.state == MazeState::Finished {
            return;
        }
        let mut neighbours = self
            .walker
            .path
            .last()
            .expect("Walker path can't be empty")
            .get_neighbours();
        fastrand::shuffle(&mut neighbours);
        let goto = neighbours
            .iter()
            .filter(|neighbour| {
                !self.is_outside(**neighbour) && !self.visited_cells.contains(&neighbour)
            })
            .next();

        match goto {
            Some(next_pos) => {
                self.walker.path.push(*next_pos);
                self.visited_cells.push(*next_pos);
                if !self.backtrack.is_empty() {
                    let track = std::mem::replace(&mut self.backtrack, Vec::new());
                    self.tracks.push(track);
                }
            }
            None => {
                self.backtrack
                    .push(self.walker.path.pop().expect("Walker path can't be empty"));
                if self.walker.path.is_empty() {
                    self.state = MazeState::Finished;
                    self.tracks
                        .push(std::mem::replace(&mut self.backtrack, Vec::new()));
                }
            }
        }
    }

    fn is_outside(&self, pos: GridPosition) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32
    }
}
