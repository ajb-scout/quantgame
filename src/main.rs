mod config;
mod history;
pub mod game;
pub mod renderers;
pub mod util;

use game::{MathAnswer, MathGame};
use std::io;
const ASCII_TITLE: [&str; 5] = [
    "   ____                   _     ___                     ",
    r"  /___ \_   _  __ _ _ __ | |_  / _ \__ _ _ __ ___   ___ ",
    r" //  / / | | |/ _` | '_ \| __|/ /_\/ _` | '_ ` _ \ / _ \",
    r"/ \_/ /| |_| | (_| | | | | |_/ /_\\ (_| | | | | | |  __/",
    r"\___,_\ \__,_|\__,_|_| |_|\__\____/\__,_|_| |_| |_|\___|",
];
// async fn timer(game_state: &mut MathGame) {
//     while game_state.get_elapsed_time_seconds() < game_state.gameconfig.timer {
//         sleep(Duration::from_secs(1));
//         game_state.current_time = Local::now();
//     }
// }

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = MathGame::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
