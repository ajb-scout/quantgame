use std::collections::HashMap;

use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{block::Title, Block, Paragraph},
    Frame,
};

use crate::{game::MathGame, util::Sign};

pub(crate) fn display_result_summary(frame: &mut Frame, area: Rect, game: &MathGame) {
    let title: Title = Title::from("Results");

    let grouped_sums: HashMap<Sign, i32> =
        game.answers
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, item| {
                *acc.entry(item.q.sign).or_insert(0) += 1;
                acc
            });

    let mut sorted_sums: Vec<(String, i32)> = grouped_sums
        .into_iter()
        .map(|f| (f.0.to_string(), f.1))
        .collect();
    sorted_sums.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by key (the String)

    let mut line_vec = vec![];
    line_vec.push(Line::from(format!("Score: {}", game.answers.len() - 1)));

    for (s, i) in sorted_sums {
        line_vec.push(Line::from(format!["{:<8}: {}", s.to_string(), i]));
    }

    frame.render_widget(
        Paragraph::new(line_vec).block(Block::bordered().title(title)),
        area,
    );
}
