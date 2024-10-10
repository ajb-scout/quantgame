mod config;
mod history;
pub mod game;
pub mod renderers;
pub mod util;
pub mod event_handlers;

use game::{MathAnswer, MathGame};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = MathGame::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
