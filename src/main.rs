mod config;
pub mod event;
pub mod event_handlers;
pub mod game;
mod history;
pub mod renderers;
pub mod tui;
pub mod util;

use event::{Event, EventHandler};
use game::{AppResult, MathAnswer, MathGame};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io;
use tui::Tui;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut terminal = ratatui::init();
//     let app_result = MathGame::default().run(&mut terminal);
//     ratatui::restore();
//     app_result
// }

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = MathGame::default();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(50);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while !app.exit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => event_handlers::handle_tick_event(&mut app),
            Event::Key(key_event) => event_handlers::handle_events(key_event, &mut app)?,
            // Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
