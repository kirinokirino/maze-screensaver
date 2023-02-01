use speedy2d::{
    color::Color,
    dimen::UVec2,
    window::{
        KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
        WindowPosition, WindowSize,
    },
    Graphics2D, Window,
};

mod maze;
use maze::{Maze, MazeState};

const STEPS_PER_DRAW: usize = 1;
const SLEEP_MS_PER_DRAW: u32 = 60;
const WINDOW_X: usize = 900;
const WINDOW_Y: usize = 600;
const WINDOW_SIZE: WindowSize =
    WindowSize::PhysicalPixels(UVec2::new(WINDOW_X as u32, WINDOW_Y as u32));
fn main() {
    let window = Window::new_with_options(
        "FLOATING",
        WindowCreationOptions::new_windowed(WINDOW_SIZE, Some(WindowPosition::Center))
            .with_decorations(false)
            .with_transparent(true),
    )
    .expect("Wasn't able to create a window!");
    let cell_size = 7.0;
    let cells_width = WINDOW_X as f32 / cell_size;
    let cells_height = WINDOW_Y as f32 / cell_size;
    window.run_loop(App::new(
        cells_width.round() as usize,
        cells_height.round() as usize,
        cell_size,
    ));
}
struct App {
    maze: Maze,
    autoplay: bool,
    step: u64,
}

impl App {
    pub fn new(width: usize, height: usize, scale: f32) -> Self {
        Self {
            maze: Maze::new(width, height, scale),
            autoplay: true,
            step: 0,
        }
    }

    pub fn reset(&mut self) {
    	let Maze { width, height, scale, .. } = self.maze;
		self.maze = Maze::new(width, height, scale);
    }
}

impl WindowHandler for App {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        self.step += 1;
        graphics.clear_screen(Color::from_gray(0.87));
        self.maze.draw(graphics, self.step);

        if self.autoplay {
        	if self.maze.state == MazeState::Finished {
				self.reset();
        	}
        	for _ in 0..STEPS_PER_DRAW {
            	self.maze.step();
            }
            std::thread::sleep(std::time::Duration::from_millis(SLEEP_MS_PER_DRAW.into()));
            helper.request_redraw();
        }
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        if let Some(key_code) = virtual_key_code {
            match key_code {
                VirtualKeyCode::U => self.maze.paths_lengths(),
                VirtualKeyCode::E => {
                	self.autoplay = !self.autoplay;
                	helper.request_redraw();
                },
                VirtualKeyCode::O => self.maze.p(),
                VirtualKeyCode::A => self.reset(),
                VirtualKeyCode::Escape => helper.terminate_loop(),
                VirtualKeyCode::Space => {
                    self.maze.step();
                    helper.request_redraw();
                }
                a => println!("Key: {a:?}, scancode: {scancode}"),
            }
        }
    }
}
