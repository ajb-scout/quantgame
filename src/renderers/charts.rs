use ratatui::{layout::{Direction, Rect}, style::{Style, Stylize}, symbols, widgets::{Axis, Bar, BarChart, BarGroup, Block, Chart, Dataset, GraphType}, Frame};

use crate::game::MathGame;


pub(crate) fn render_sparkline_from(
    frame: &mut Frame,
    area: Rect,
    game: &MathGame,
    orientation: Direction,
    title: String
) {
    let mut d1: Vec<(f64, f64)> = vec![];

    for (i, q) in game.answers.iter().enumerate() {
        d1.push((i as f64, q.duration_m as f64));
    }

    let colors = crate::util::create_gradient(&d1.iter().map(|f| f.1 as f32).collect::<Vec<f32>>());

    let bars: Vec<Bar> = game.answers
        .iter()
        .enumerate()
        .map(|(u, f)| {
            Bar::default()
                .value(f.duration_m as u64)
                .style(colors[u])
                .text_value(format!("{:<6}ms ", f.duration_m))
                .value_style(colors[u])
        })
        .collect();
    let datasets = vec![
        // Scatter chart
        Dataset::default()
            .name("Times")
            // .marker(symbols)
            .marker(symbols::Marker::Block)
            .graph_type(GraphType::Bar)
            .style(Style::default().cyan())
            .data(&d1),
        // Line chart
    ];
    let bc = BarChart::default()
        .block(Block::bordered().title("Chart"))
        .bar_width(1)
        .bar_gap(0)
        .direction(orientation)
        .value_style(Style::new().red().bold())
        .label_style(Style::new().white())
        .data(BarGroup::default().bars(&bars));
    // .data(BarGroup::default().bars(&[1,2,3].map(|f| Bar::from(Bar::default().value(f)))));
    // Create the X axis and define its properties
    let binding = d1.len().to_string();
    let x_axis = Axis::default()
        .title("X Axis".red())
        .style(Style::default().white())
        .bounds([0.0, d1.len() as f64])
        .labels(["0.0", &binding]);

    // Create the Y axis and define its properties
    let binding = d1
        .iter()
        .map(|f| f.1)
        .collect::<Vec<f64>>()
        .into_iter()
        .fold(0. / 0., f64::max)
        .to_string();
    let y_axis = Axis::default()
        .title("Y Axis".red())
        .style(Style::default().white())
        .bounds([
            0.0,
            d1.iter()
                .map(|f| f.1)
                .collect::<Vec<f64>>()
                .into_iter()
                .fold(0. / 0., f64::max),
        ])
        .labels(["0.0", &binding]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(Block::new().title(title))
        .x_axis(x_axis)
        .y_axis(y_axis);

    frame.render_widget(bc, area);
}

pub fn render_score_history_graph(frame: &mut Frame, area: Rect, game: &MathGame) {
    let mut d1: Vec<(f64, f64)> = vec![];

    for (i, q) in game.game_history.history.iter().enumerate() {
        d1.push((i as f64, q.score as f64));
    }

    let datasets = vec![
        // Scatter chart
        Dataset::default()
            .name("Times")
            // .marker(symbols)
            .marker(symbols::Marker::Block)
            .graph_type(GraphType::Bar)
            .style(Style::default().cyan())
            .data(&d1),
        // Line chart
    ];

    // Create the X axis and define its properties
    let binding = d1.len().to_string();
    let x_axis = Axis::default()
        .title("X Axis".red())
        .style(Style::default().white())
        .bounds([0.0, d1.len() as f64])
        .labels(["0.0", &binding]);

    // Create the Y axis and define its properties
    let binding = d1
        .iter()
        .map(|f| f.1)
        .collect::<Vec<f64>>()
        .into_iter()
        .fold(0. / 0., f64::max)
        .to_string();
    let y_axis = Axis::default()
        .title("Y Axis".red())
        .style(Style::default().white())
        .bounds([
            0.0,
            d1.iter()
                .map(|f| f.1)
                .collect::<Vec<f64>>()
                .into_iter()
                .fold(0. / 0., f64::max),
        ])
        .labels(["0.0", &binding]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(Block::new().title("Chart"))
        .x_axis(x_axis)
        .y_axis(y_axis);

    frame.render_widget(chart, area);
}


