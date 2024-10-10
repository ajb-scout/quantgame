pub mod table;
pub mod charts;
pub mod screen;

use ratatui::{layout::{Alignment, Constraint, Direction, Layout}, style::Stylize, symbols::border, text::{Line, Span, Text}, widgets::{Block, Paragraph, Wrap}, Frame};

use crate::game::MathGame;

pub fn render_game_splash(frame: &mut Frame, _game: &mut MathGame) {
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
    for i in crate::util::ASCII_TITLE.iter().copied() {
        title_vec.push(Line::from(i));
    }

    // build text objects
    let splash_text = Text::from(title_vec).alignment(Alignment::Left);
    let options_text = (Span::from("S").underlined().bold() + Span::from("tart"))
        // + (Span::from("S") + Span::from("e").underlined().bold() + Span::from("ttings"))
        + (Span::from("H").underlined().bold() + Span::from("istory"))
        + (Span::from("Q").underlined().bold() + Span::from("uit"));


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

pub fn render_end_splash(frame: &mut Frame, game: &mut MathGame) {
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

    screen::display_result_summary(frame, inner_layout[0], &game);
    table::render_table_from_questions(frame, inner_layout[1], game);
    charts::render_question_time_barchart(frame, layout[1], &game, Direction::Vertical, "Results".to_string());
}
// fn draw_end_splash()

pub fn render_history_splash(frame: &mut Frame, game: &mut MathGame) {
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
    
    // let qtable = render_table_from_questions(history_answers);
    table::render_table_from_history(frame, layout[0], game);
    table::render_table_from_questions(frame, layout[1], game);
    charts::render_question_time_barchart(frame, layout[2], game, Direction::Horizontal, "Times".to_string());
    charts::render_score_history_graph(frame, outer_layout[1], game);
}

