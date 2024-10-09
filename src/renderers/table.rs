use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Cell, Row, Table, TableState},
    Frame,
};

use crate::game::MathGame;

pub fn render_table_from_questions(frame: &mut Frame, area: Rect, game: &mut MathGame) {
    let header = ["Question", "Answer", "Time", "120s Pace"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(2);
    let v = game
        .answers
        .iter()
        .map(|f| f.duration_m as f32)
        .collect::<Vec<f32>>();

    let colors = crate::util::create_gradient(&v);

    let mut rows: Vec<Row> = vec![];
    let mut running_total: i64 = 0;
    for (x, i) in game.answers.iter().enumerate() {
        running_total += i.duration_m;
        let qstring = i.string_representation.to_string();
        let astring = i.q.answer.to_string();
        let mut tstring = i.duration_m.to_string();
        let running_average: f64 = 120000.0 / (running_total / (x as i64 + 1)) as f64;
        let rstring = running_average.to_string();
        // tstring.insert(tstring.len() - 3, '.');
        tstring.push_str(" s");
        // let msstring = i.question_answer.unwrap().duration_since(i.question_start).as_millis().

        rows.push(Row::new(vec![
            Line::from(qstring),
            Line::from(astring),
            Line::from(tstring).style(Style::new().fg(colors[x])),
            Line::from(rstring),
        ]));
    }

    let bar = " █ ";
    let table = Table::new(
        rows,
        [
            // + 1 is for padding.
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .block(
        Block::bordered()
            .title("Table")
            .border_type(BorderType::Rounded),
    )
    .highlight_style(Style::new().bg(Color::DarkGray))
    .highlight_symbol(">>")
    .column_spacing(1)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]));
    frame.render_stateful_widget(table, area, &mut game.result_table_state);
}

pub fn render_table_from_history(
    frame: &mut Frame,
    area: Rect,
    game: &mut MathGame,
) {
    let header = ["#", "Date", "Score"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(2);

    let mut rows: Vec<Row> = vec![];
    for (x, i) in game.game_history.history.iter().enumerate() {
        rows.push(Row::new(vec![
            Line::from(x.to_string()),
            Line::from(i.game_intant.to_string()),
            Line::from(i.score.to_string()),
        ]));
    }

    let bar = " █ ";

    let table = Table::new(
        rows,
        [
            // + 1 is for padding.
            Constraint::Length(6),
            Constraint::Length(10),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .block(
        Block::bordered()
            .title("Table")
            .border_type(BorderType::Rounded),
    )
    .highlight_style(Style::new().bg(Color::DarkGray))
    .highlight_symbol(">>")
    .column_spacing(1)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]));


    frame.render_stateful_widget(table, area, &mut game.history_table_state);
}
