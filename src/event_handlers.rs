use std::io;

use chrono::Local;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::game::{GameState, MathGame, MathQuestion};

//handle game tick, used to check if timeout has occured
pub fn handle_tick_event(game: &mut MathGame) {
    if game.get_elapsed_time_seconds() > game.gameconfig.timer
        && game.gamestate == GameState::Inprogress
    {
        game.handle_game_end(true);
    }
}

//top level event handler, subordinates to other handlers depending on game state
pub fn handle_events(key_event: KeyEvent, game: &mut MathGame) -> io::Result<()> {
    match game.gamestate {
        GameState::Setup => handle_key_event_splash(game, key_event),
        GameState::Inprogress => handle_key_event_game(game, key_event),
        GameState::EndingSplash => handle_end_event_splash(game, key_event),
        GameState::HistorySplash => handle_key_event_history(game, key_event),
        GameState::SettingsSpash => handle_key_event_game(game, key_event),
    }
    Ok(())
}

fn handle_end_event_splash(game: &mut MathGame, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => game.exit(),
        KeyCode::Char('d') => game.handle_return_to_splash(),

        KeyCode::Up => game.result_table_state.select_previous(),
        KeyCode::Down => game.result_table_state.select_next(),
        _ => {}
    }
}

fn handle_key_event_history(game: &mut MathGame, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => game.exit(),
        KeyCode::Char('d') => game.handle_return_to_splash(),

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

    // let solved = game.input.parse::<i32>() == Ok(game.current_question.answer);
    // // correct answer,
    // if solved {
    //     game.score += 1;
    //     game.input.clear();
    //     game.current_question = MathQuestion::generate_new_question(&game.gameconfig.qr);
    // }
}

fn handle_key_event_game(game: &mut MathGame, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => game.exit(),
        KeyCode::Char('r') => game.handle_game_restart(),
        KeyCode::Char('e') => game.handle_game_end(true),
        KeyCode::Char('d') => game.handle_return_to_splash(),

        KeyCode::Backspace => {
            let _ = &game.input.pop();
        }
        KeyCode::Delete => {
            let _ = &game.input.pop();
        }

        _ => game.input.push_str(&key_event.code.to_string()),
    };
    // check to see if most recent input has solved the question
    let solved = game.input.parse::<i32>() == Ok(game.current_question.answer);
    if solved {
        game.score += 1;
        game.input.clear();
        game.current_question.question_answer = Some(Local::now());
        game.questions.push(game.current_question);
        game.current_question = MathQuestion::generate_new_question(&game.gameconfig.qr);
    }
}
