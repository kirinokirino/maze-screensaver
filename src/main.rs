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

mod config;
use config::SETTINGS;

fn main() {
    let window_size = WindowSize::PhysicalPixels(UVec2::new(
        SETTINGS.read().unwrap().window_width as u32,
        SETTINGS.read().unwrap().window_height as u32,
    ));
    let window = Window::new_with_options(
        "FLOATING",
        WindowCreationOptions::new_windowed(window_size, Some(WindowPosition::Center))
            .with_decorations(false)
            .with_transparent(true),
    )
    .expect("Wasn't able to create a window!");
    let cell_size = 7.0;
    let cells_width = SETTINGS.read().unwrap().window_width as f32 / cell_size;
    let cells_height = SETTINGS.read().unwrap().window_height as f32 / cell_size;
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
        let Maze {
            width,
            height,
            scale,
            ..
        } = self.maze;
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
            for _ in 0..SETTINGS.read().unwrap().steps_per_draw {
                self.maze.step();
            }
            std::thread::sleep(std::time::Duration::from_millis(
                SETTINGS.read().unwrap().sleep_ms_per_frame.into(),
            ));
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
                }
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
