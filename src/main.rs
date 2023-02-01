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
use maze::Maze;

const WINDOW_X: usize = 480;
const WINDOW_Y: usize = 360;
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
    let cell_size = 10.0;
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
}

impl App {
    pub fn new(width: usize, height: usize, scale: f32) -> Self {
        Self {
            maze: Maze::new(width, height, scale),
            autoplay: true,
        }
    }
}

impl WindowHandler for App {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_gray(1.0));
        self.maze.draw(graphics);

        if self.autoplay {
            self.maze.step();
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
