mod config;
pub mod event_handlers;
pub mod game;
mod history;
pub mod renderers;
pub mod util;

use game::{MathAnswer, MathGame};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = MathGame::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
