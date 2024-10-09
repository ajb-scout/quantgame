pub mod table;
pub mod misc;
pub mod charts;

use rand_distr::num_traits::ToPrimitive;
use ratatui::{layout::{Alignment, Constraint, Direction, Layout}, style::Stylize, symbols::border, text::{Line, Span, Text}, widgets::{Block, Paragraph, Widget, Wrap}, Frame};

use crate::game::MathGame;

pub fn display_test_splash(){
    Block::new();
}

pub fn render_game_splash(frame: &mut Frame, game: &mut MathGame) {
    let outer_layout = Layout::new(
        Direction::Vertical,
        vec![
            Constraint::Percentage(10),
            Constraint::Percentage(70),
            Constraint::Percentage(20),
        ],
    )
    .split(frame.area());

    // build the ascii string from the title
    let mut title_vec = vec![];
    for i in crate::ASCII_TITLE.iter().copied() {
        title_vec.push(Line::from(i));
    }

    // build text objects
    let splash_text = Text::from(title_vec).alignment(Alignment::Left);
    let options_text = (Span::from("S").underlined().bold() + Span::from("tart"))
        + (Span::from("Q").underlined().bold() + Span::from("uit"))
        + (Span::from("S") + Span::from("e").underlined().bold() + Span::from("ttings"))
        + (Span::from("H").underlined().bold() + Span::from("istory"));

    // build splash para
    let splash_para = Paragraph::new(splash_text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    let options_para = Paragraph::new(options_text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    // render the widgets
    frame.render_widget(Block::bordered().border_set(border::ROUNDED), frame.area());
    frame.render_widget(splash_para, outer_layout[1]);
    frame.render_widget(options_para, outer_layout[2]);
}

pub fn draw_end_splash(frame: &mut Frame, game: &mut MathGame) {
    let border_layout =
        Layout::new(Direction::Vertical, vec![Constraint::Percentage(100)]).split(frame.area());

    let layout = Layout::new(
        Direction::Vertical,
        vec![Constraint::Percentage(60), Constraint::Percentage(40)],
    )
    .split(border_layout[0]);
    let inner_layout = Layout::new(
        Direction::Horizontal,
        vec![Constraint::Percentage(30), Constraint::Percentage(70)],
    )
    .split(layout[0]);

    // build text objects
    let splash_text = Text::from(
        Span::from("Score: ") + Span::from(game.score.to_string()).bold().underlined(),
    )
    .alignment(Alignment::Left);

    misc::display_result_summary(frame, inner_layout[0], &game);
    table::render_table_from_questions(frame, inner_layout[1], game);

    // render the widgets
    // frame.render_widget(Block::bordered().border_set(border::ROUNDED), border_layout[0]);
    // frame.render_widget(table::display_result_summary(&game.answers), inner_layout[0]);
    // frame.render_stateful_widget(table, inner_layout[1], &mut game.result_table_state);

    charts::render_sparkline_from(frame, layout[1], &game, Direction::Vertical, "Results".to_string());
}
// fn draw_end_splash()

pub fn draw_history_splash(frame: &mut Frame, game: &mut MathGame) {
    let outer_layout = Layout::new(
        Direction::Vertical,
        vec![Constraint::Percentage(80), Constraint::Percentage(20)],
    )
    .split(frame.area());

    let layout = Layout::new(
        Direction::Horizontal,
        vec![
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ],
    )
    .split(outer_layout[0]);

    //get the state as the index unless nothing selected in which case set to zero
    let mut selected_index_no_oob = game.history_table_state.selected().unwrap_or(0) as u32;
    if game.game_history.history.len() == 0 {
        selected_index_no_oob = 0;
    } else if selected_index_no_oob + 1
        >= game.game_history.history.len().try_into().unwrap_or(0)
    {
        if game.game_history.history.len() == 0 {
            selected_index_no_oob = 0
        } else {
            selected_index_no_oob = game.game_history.history.len() as u32 - 1;
        }
    }

    let selected_history = game
        .game_history
        .history
        .get(selected_index_no_oob.to_usize().unwrap());
    let history_answers = match selected_history {
        Some(x) => &x.answers,
        None => &vec![],
    };

    // let qtable = render_table_from_questions(history_answers);
    table::render_table_from_questions(frame, layout[1], game);
    table::render_table_from_history(frame, layout[0], game);

    // frame.render_stateful_widget(table, layout[0], &mut self.history_table_state);
    // frame.render_widget(qtable, layout[1]);
    charts::render_sparkline_from(frame, layout[2], game, Direction::Horizontal, "Best Scores".to_string());

    charts::render_score_history_graph(frame, outer_layout[1], game);
}

