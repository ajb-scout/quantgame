use std::io;

use chrono::Local;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::game::{GameState, MathGame, MathQuestion};

pub fn handle_events(game: &mut MathGame) -> io::Result<()> {
    match event::read()? {
        // it's important to check that the event is a key press event as
        // crossterm also emits key release and repeat events on Windows.
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            match game.gamestate {
                GameState::Setup => handle_key_event_splash(game,key_event),
                GameState::Inprogress => handle_key_event_game(game,key_event),
                GameState::EndingSplash => handle_end_event_splash(game, key_event),
                GameState::HistorySplash => handle_key_event_history(game,key_event),
                GameState::SettingsSpash => handle_key_event_game(game, key_event),
            }
        }
        _ => {}
    };
    Ok(())
}
fn handle_end_event_splash(game: &mut MathGame, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => game.exit(),
        KeyCode::Up => game.result_table_state.select_previous(),
        KeyCode::Down => game.result_table_state.select_next(),
        _ => {}
    }
}

fn handle_key_event_history(game: &mut MathGame, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => game.exit(),
        KeyCode::Up => game.history_table_state.select_previous(),
        KeyCode::Down => game.history_table_state.select_next(),
        _ => {}
    }
}

fn handle_key_event_splash(game: &mut MathGame, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => game.exit(),
        KeyCode::Char('s') => {
            game.gamestate = GameState::Inprogress;
            game.start_time = Local::now();
            game.current_question = MathQuestion::generate_new_question(&game.gameconfig.qr)
        }
        KeyCode::Char('h') => {
            game.gamestate = GameState::HistorySplash;
        }

        KeyCode::Delete => {
            game.input.pop();
        }
        _ => {}
    }

    let solved = game.input.parse::<i32>() == Ok(game.current_question.answer);
    // correct answer
    if solved {
        game.score += 1;
        game.input.clear();
        game.current_question = MathQuestion::generate_new_question(&game.gameconfig.qr);
    }
}

fn handle_key_event_game(game: &mut MathGame, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => game.exit(),
        KeyCode::Char('r') => game.handle_game_restart(),
        KeyCode::Backspace => {
            let _ = &game.input.pop();
        },
        KeyCode::Delete => {
            let _ = &game.input.pop();
        },

        _ => game.input.push_str(&key_event.code.to_string()),
    };

    let solved = game.input.parse::<i32>() == Ok(game.current_question.answer);
    if solved {
        game.score += 1;
        game.input.clear();
        game.current_question.question_answer = Some(Local::now());
        game.questions.push(game.current_question);
        game.current_question = MathQuestion::generate_new_question(&game.gameconfig.qr);
    }
}