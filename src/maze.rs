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
        let half_cell_size = Vector2::new(scale / 2.0, scale / 2.0);

		let trail = Color::from_hex_rgb(0xbab19d);
        
        for path_segment in self.path.windows(2) {
            graphics.draw_line(
                path_segment[0].as_vector2(scale) + half_cell_size,
                path_segment[1].as_vector2(scale) + half_cell_size,
                2.0,
                trail,
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
pub enum MazeState {
    Generating,
    Finished,
}

pub struct Maze {
    pub scale: f32,
    pub width: usize,
    pub height: usize,

    start: GridPosition,
    finish: GridPosition,

    walker: Walker,
    visited_cells: Vec<GridPosition>,
    backtrack: Vec<GridPosition>,
    tracks: Vec<Vec<GridPosition>>,

    pub state: MazeState,
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

    pub fn draw(&self, graphics: &mut Graphics2D, step: u64) {
        let cell_size = Vector2::new(self.scale, self.scale);
        let half_cell_size = cell_size / 2.0;

        let padding = Vector2::new(0.1, 0.1);

        if false {
            for cell in self.visited_cells.iter() {
                let pos = cell.as_vector2(self.scale);
                graphics.draw_rectangle(
                    Rectangle::new(pos + padding, pos + cell_size - padding - padding),
                    Color::from_gray(0.8),
                )
            }
        }

        fastrand::seed(999999);
        for track in self.tracks.iter() {
            let track_color = Color::from_rgb(
                fastrand::f32() * 0.2 + 0.6,
                fastrand::f32() * 0.2 + 0.6,
                fastrand::f32() * 0.4 + 0.2,
            );
            for path_segment in track.windows(2) {
                graphics.draw_line(
                    path_segment[0].as_vector2(self.scale) + half_cell_size,
                    path_segment[1].as_vector2(self.scale) + half_cell_size,
                    self.scale / 2.0,
                    track_color,
                );
            }
        }
        fastrand::seed(step);

        if false {
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
        }

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
            .iter().find(|neighbour| !self.is_outside(**neighbour) && !self.visited_cells.contains(neighbour));

        match goto {
            Some(next_pos) => {
                self.walker.path.push(*next_pos);
                self.visited_cells.push(*next_pos);
                if !self.backtrack.is_empty() {
                    let track = std::mem::take(&mut self.backtrack);
                    self.tracks.push(track);
                }
            }
            None => {
                self.backtrack
                    .push(self.walker.path.pop().expect("Walker path can't be empty"));
                if self.walker.path.is_empty() {
                    self.state = MazeState::Finished;
                    self.tracks
                        .push(std::mem::take(&mut self.backtrack));
                }
            }
        }
    }

    pub fn paths_lengths(&self) {
        use intmap::IntMap;
        use itertools::sorted;
        let mut map: IntMap<u64> = IntMap::new();
        for path in self.tracks.iter() {
            let length = path.len();
            let x = map.get_mut(length.try_into().unwrap());
            match x {
                Some(x) => *x += 1,
                None => {
                    map.insert(length.try_into().unwrap(), 1);
                }
            }
        }

        let mut data = String::new();
        for (x, y) in sorted(map.drain()) {
            println!("{y}\t  tracks of length\t {x}");
            if data.is_empty() {
                data = format!("{y}");
            } else {
                data = format!("{data}, {y}");
            }
        }
        println!("{data}");
    }
    
	pub fn p(&mut self) {
		self.tracks = self.tracks.clone().into_iter().filter(|track| track.len() > 10).collect();
	}
	
    fn is_outside(&self, pos: GridPosition) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32
    }
}
