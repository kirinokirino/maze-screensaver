use speedy2d::{
    color::Color,
    dimen::UVec2,
    window::{WindowCreationOptions, WindowHandler, WindowPosition, WindowSize},
    Window,
};

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
    window.run_loop(App::new());
}

struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }
}

impl WindowHandler for App {
    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut speedy2d::Graphics2D,
    ) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
}
