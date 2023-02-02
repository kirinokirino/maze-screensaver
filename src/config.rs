use configparser::ini::Ini;
use lazy_static::lazy_static;

use std::default::Default;
use std::error::Error;
use std::sync::RwLock;
use std::thread::panicking;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::new());
}

pub struct Config {
    pub sleep_ms_per_frame: u32,
    pub steps_per_draw: usize,
    pub window_width: usize,
    pub window_height: usize,
    pub decorations: bool,
}

impl Config {
    fn new() -> Self {
        let mut config = Self::default();
        if let Err(error) = config.reload() {
            eprintln!("{error}");
            #[cfg(debug_assertions)]
            panic!();
        }
        config
    }

    pub fn reload(&mut self) -> Result<(), Box<dyn Error>> {
        let path = "config.ini";
        let mut ini = Ini::new();
        if ini.load(path).is_ok() {
            let default_section = "default";
            self.sleep_ms_per_frame = ini
                .getuint(default_section, "sleep_ms_per_frame")?
                .unwrap_or(60)
                .try_into()
                .unwrap_or(60);
            self.steps_per_draw = ini
                .getuint(default_section, "steps_per_draw")?
                .unwrap_or(1)
                .try_into()
                .unwrap_or(1);
            self.window_width = ini
                .getuint(default_section, "window_width")?
                .unwrap_or(480)
                .try_into()
                .unwrap();
            self.window_height = ini
                .getuint(default_section, "window_height")?
                .unwrap_or(360)
                .try_into()
                .unwrap();
            self.decorations = ini
                .getbool(default_section, "decorations")?
                .unwrap_or(true)
                .try_into()
                .unwrap();
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sleep_ms_per_frame: 60,
            steps_per_draw: 1,
            window_width: 480,
            window_height: 360,
            decorations: true,
        }
    }
}
