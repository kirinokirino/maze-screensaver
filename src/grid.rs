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

pub struct Grid {
    scale: f32,
    width: usize,
    height: usize,

    start: GridPosition,
    finish: GridPosition,
}

impl Grid {
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
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        let cell_size = Vector2::new(self.scale as f32, self.scale as f32);
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

    fn is_outside(&self, pos: GridPosition) -> bool {
        pos.x < 0 && pos.y < 0 && pos.x >= self.width as i32 && pos.y >= self.height as i32
    }
}
