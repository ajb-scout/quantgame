mod config;
mod history;

use crate::config::GameConfiguration;
use chrono::prelude::*;
use config::QuestionRanges;
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use history::{render_table_from_history, GameHistory, GameRecord};
use rand::Rng;
use rand_distr::num_traits::ToPrimitive;
use rand_distr::{Distribution, Normal};

use ratatui::style::{Color, Style};
use ratatui::widgets::{
    Axis, Bar, BarChart, BarGroup, BorderType, Borders, Chart, Dataset, GraphType, ScrollbarState,
    Sparkline,
};
use ratatui::{layout, symbols};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, time::Instant};
use std::{io, thread};

use ratatui::{
    buffer::Buffer,
    crossterm,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span, Text, ToSpan},
    widgets::{
        block::{Position, Title},
        Block, Cell, Paragraph, Row, Table, TableState, Widget, Wrap,
    },
    DefaultTerminal, Frame,
};

use std::time::Duration;

const ascii_title: [&str; 5] = [
    "   ____                   _     ___                     ",
    r"  /___ \_   _  __ _ _ __ | |_  / _ \__ _ _ __ ___   ___ ",
    r" //  / / | | |/ _` | '_ \| __|/ /_\/ _` | '_ ` _ \ / _ \",
    r"/ \_/ /| |_| | (_| | | | | |_/ /_\\ (_| | | | | | |  __/",
    r"\___,_\ \__,_|\__,_|_| |_|\__\____/\__,_|_| |_| |_|\___|",
];
#[derive(Debug)]
pub struct MathGame {
    current_question: MathQuestion,
    // game_is_started: bool,
    exit: bool,
    input: String,
    score: i32,
    start_time: DateTime<Local>,
    current_time: DateTime<Local>,
    answered_questions: Vec<MathQuestion>,
    gamestate: GameState,
    gameconfig: GameConfiguration,
    result_table_state: TableState,
    scrollbar_state: ScrollbarState,
    game_history: GameHistory,
    history_table_state: TableState,
}

impl Default for MathGame {
    fn default() -> Self {
        let config = GameConfiguration::default();
        let first_question = MathQuestion::generate_new_question(&config.qr);
        Self {
            current_question: first_question,
            // game_is_started: Default::default(),
            exit: Default::default(),
            input: Default::default(),
            score: Default::default(),
            start_time: Local::now(),
            current_time: Local::now(),
            answered_questions: vec![],
            gamestate: GameState::Setup,
            gameconfig: config,
            result_table_state: TableState::default().with_selected(0),
            scrollbar_state: ScrollbarState::default(),
            game_history: GameHistory::new("results.json").unwrap(),
            history_table_state: TableState::default().with_selected(0),
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MathQuestion {
    lhs: i32,
    rhs: i32,
    answer: i32,
    sign: Sign,
    question_start: DateTime<Local>,
    question_answer: Option<DateTime<Local>>,
}

impl MathQuestion {
    fn generate_lhs_rhs(qr: &QuestionRanges, sign: &Sign) -> (i32, i32) {
        return match sign {
            Sign::Multiply => (
                rand::thread_rng().gen_range(qr.mult_lhs_lower..qr.mult_lhs_upper),
                rand::thread_rng().gen_range(qr.mult_rhs_lower..qr.mult_rhs_upper),
            ),
            Sign::Add => (
                rand::thread_rng().gen_range(qr.add_lower..qr.add_upper),
                rand::thread_rng().gen_range(qr.add_lower..qr.add_upper),
            ),
            Sign::Subtract => {
                let lhs = rand::thread_rng().gen_range(qr.add_lower..qr.add_upper);
                let rhs = rand::thread_rng().gen_range(qr.add_lower..qr.add_upper);
                if lhs > rhs {
                    return (lhs, rhs);
                }
                (rhs, lhs)
            }
            Sign::Divide => {
                let lhs = rand::thread_rng().gen_range(qr.mult_lhs_lower..qr.mult_lhs_upper);
                let rhs = rand::thread_rng().gen_range(qr.mult_rhs_lower..qr.mult_rhs_upper);
                let ans = lhs * rhs;
                (ans, lhs)
        },
        };
    }

    pub fn generate_new_question(qr: &QuestionRanges) -> MathQuestion {
        let sign = match rand::thread_rng().gen_range(1..5) {
            1 => Sign::Add,
            2 => Sign::Subtract,
            3 => Sign::Multiply,
            4 => Sign::Divide,
            _ => panic!("Shouldn't be able to generate this"),
        };

        let lhs_rhs = Self::generate_lhs_rhs(qr, &sign);
        let answer: i32 = apply_sign(&sign, lhs_rhs.0, lhs_rhs.1);
        let new_question = MathQuestion {
            lhs: lhs_rhs.0,
            rhs: lhs_rhs.1,
            answer: answer,
            sign: sign,
            question_start: Local::now(),
            question_answer: Option::None,
        };
        return new_question;
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Sign {
    Multiply,
    Add,
    Subtract,
    Divide,
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sign::Multiply => write!(f, "x"),
            Sign::Add => write!(f, "+"),
            Sign::Subtract => write!(f, "-"),
            Sign::Divide => write!(f, "/"),
        }
    }
}
#[derive(Debug, PartialEq)]
enum GameState {
    Setup,
    Inprogress,
    EndingSplash,
    HistorySplash,
    SettingsSpash,
}

fn match_sign(sign: &Sign) -> char {
    match sign {
        Sign::Multiply => 'x',
        Sign::Add => '+',
        Sign::Subtract => '-',
        Sign::Divide => '/',
    }
}

fn apply_sign(sign: &Sign, lhs: i32, rhs: i32) -> i32 {
    return match sign {
        Sign::Multiply => lhs * rhs,
        Sign::Add => lhs + rhs,
        Sign::Subtract => lhs - rhs,
        Sign::Divide => lhs / rhs,
    };
}

impl Widget for &MathGame {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Quant Game".bold());

        let score = Title::from(Line::from(vec![
            " Score:  ".into(),
            self.score.to_string().bold(),
            "  Elapsed:  ".into(),
            (self.current_time - self.start_time)
                .num_seconds()
                .to_string()
                .bold(),
            " ".into(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                score
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::ROUNDED);

        let solved = self.input.parse::<i32>() == Ok(self.current_question.answer);
        let mut input_line = Span::from(self.input.clone().white());
        if solved {
            input_line = Span::from(self.input.clone().green());
        }

        let counter_text = Text::from(vec![
            Line::from(vec![format!("Question {}: ", self.score + 1).yellow()]),
            Line::from(vec![
                self.current_question.lhs.to_string().into(),
                " ".into(),
                match_sign(&self.current_question.sign).to_string().into(),
                " ".into(),
                self.current_question.rhs.to_string().into(),
                " = ".into(),
                // self.current_question.answer.to_string().into(),
                input_line,
            ]),
        ]);

        Paragraph::new(counter_text)
            .alignment(Alignment::Center)
            .block(block)
            .render(area, buf);
    }
}

fn create_gradient(colors: &[f32]) -> Vec<Color> {
    if colors.is_empty() {
        return Vec::new();
    }

    // Find min and max values
    let min_value = *colors
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_value = *colors
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // Create gradient colors
    colors
        .iter()
        .map(|&value| {
            let normalized = (value - min_value) / (max_value - min_value);

            // Define RGB components based on normalized value
            let red: u8;
            let green: u8;
            if normalized <= 0.5 {
                // Transition from green to yellow
                red = (normalized * 2.0 * 255.0) as u8; // Scale to 0-255
                green = 255; // Maximum green
            } else {
                // Transition from yellow to red
                red = 255; // Maximum red
                green = (255.0 - (normalized - 0.5) * 2.0 * 255.0) as u8; // Scale down green
            }

            Color::Rgb(red, green, 0)
        })
        .collect()
}

fn render_table_from_questions(qs: &Vec<MathQuestion>) -> Table {
    let header = ["Question", "Answer", "Time", "120s Pace"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(2);
    let v = qs
        .iter()
        .map(|i| {
            (i.question_answer.unwrap_or(Local::now()) - i.question_start).num_milliseconds() as f32
        })
        .collect::<Vec<f32>>();

    let colors = create_gradient(&v);

    let mut rows: Vec<Row> = vec![];
    let mut running_total: i64 = 0;
    for (x, i) in qs.iter().enumerate() {
        let num_milis = (i.question_answer.unwrap_or(Local::now()) - i.question_start).num_milliseconds();
        running_total += num_milis;
        let qstring = i.lhs.to_string() + " " + &i.sign.to_string() + " " + &i.rhs.to_string();
        let astring = i.answer.to_string();
        let mut tstring = num_milis.to_string();
        let running_average: f64 = 120000.0 / (running_total / (x as i64 + 1)) as f64;
        let rstring = running_average.to_string();
        // tstring.insert(tstring.len() - 3, '.');
        tstring.push_str(" s");
        // let msstring = i.question_answer.unwrap().duration_since(i.question_start).as_millis().

        rows.push(Row::new(vec![
            Line::from(qstring),
            Line::from(astring),
            Line::from(tstring).style(Style::new().fg(colors[x])),
            Line::from(rstring)
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
    return table;
}
fn render_score_history_graph(
    frame: &mut Frame,
    area: Rect,
    history: &Vec<GameRecord>,
) {
    let mut d1: Vec<(f64, f64)> = vec![];

    for (i, q) in history.iter().enumerate() {
        d1.push((
            i as f64,
            q.score as f64
        ));
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



//TODO - can I change this so that it no longer renders in place and instrad returns a chart object?
fn render_sparkline_from(
    frame: &mut Frame,
    area: Rect,
    qs: &Vec<MathQuestion>,
    orientation: Direction,
) {
    let mut d1: Vec<(f64, f64)> = vec![];

    for (i, q) in qs.iter().enumerate() {
        d1.push((
            i as f64,
            ((q.question_answer.unwrap()) - (q.question_start)).num_milliseconds() as f64,
        ));
    }

    let colors = create_gradient(&d1.iter().map(|f| f.1 as f32).collect::<Vec<f32>>());

    let bars: Vec<Bar> = qs
        .iter()
        .enumerate()
        .map(|(u, f)| {
            Bar::default()
                .value((f.question_answer.unwrap() - f.question_start).num_milliseconds() as u64)
                .style(colors[u])
                .text_value(format!(
                    "{:<6}ms ",
                    (f.question_answer.unwrap() - f.question_start).num_milliseconds()
                ))
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

    frame.render_widget(bc, area);
}

fn generate_random_durations(total_duration: Duration, count: i32) -> Vec<Duration> {
    let count = count.max(0);
    let mean = total_duration.as_nanos() as f64 / count as f64; // Average duration in nanoseconds
    let std_dev = mean / 0.2; // Standard deviation (adjust as needed)

    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rand::thread_rng();

    let mut durations: Vec<u64> = Vec::new();

    for _ in 0..count {
        let duration = normal.sample(&mut rng).round() as u64;

        // Ensure the duration is at least 1 nanosecond
        let valid_duration = duration.max(1);
        durations.push(valid_duration);
    }

    // Convert to Duration and return
    durations
        .into_iter()
        .map(|nanos| Duration::new(0, nanos as u32))
        .collect()
}

impl MathGame {
    fn draw_splash(&self, frame: &mut Frame) {
        let outer_layout: std::rc::Rc<[Rect]> = Layout::new(
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
        for i in ascii_title.iter().copied() {
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

    fn draw_history_splash(&mut self, frame: &mut Frame) {
        let outer_layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Percentage(80), Constraint::Percentage(20)],
        ).split(frame.area());

        let layout: std::rc::Rc<[Rect]> = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ],
        )
        .split(outer_layout[0]);

        let mut selected_index_no_oob = self.history_table_state.selected().unwrap_or(0) as u32;
        if selected_index_no_oob + 1 >= self.game_history.history.len().try_into().unwrap() {
            selected_index_no_oob = self.game_history.history.len() as u32 - 1;
        }

        let table = render_table_from_history(&self.game_history);
        let qtable = render_table_from_questions(
            &self.game_history.history[selected_index_no_oob.to_usize().unwrap()].answers,
        );

        frame.render_stateful_widget(table, layout[0], &mut self.history_table_state);
        frame.render_widget(qtable, layout[1]);
        render_sparkline_from(
            frame,
            layout[2],
            &self.game_history.history[selected_index_no_oob.to_usize().unwrap()].answers,
            Direction::Horizontal,
        );

        render_score_history_graph(frame, outer_layout[1], &self.game_history.history);
    }

    fn draw_end_splash(&mut self, frame: &mut Frame) {
        let border_layout =
            Layout::new(Direction::Vertical, vec![Constraint::Percentage(100)]).split(frame.area());

        let layout: std::rc::Rc<[Rect]> = Layout::new(
            Direction::Vertical,
            vec![Constraint::Percentage(60), Constraint::Percentage(40)],
        )
        .split(border_layout[0]);
        let inner_layout = Layout::new(
            Direction::Horizontal,
            vec![Constraint::Percentage(100), Constraint::Percentage(0)],
        )
        .split(layout[1]);

        // build text objects
        let splash_text = Text::from(
            Span::from("Score: ") + Span::from(self.score.to_string()).bold().underlined(),
        )
        .alignment(Alignment::Left);

        // build splash para
        let splash_para = Paragraph::new(splash_text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        let table = render_table_from_questions(&self.answered_questions);

        // render the widgets
        // frame.render_widget(Block::bordered().border_set(border::ROUNDED), border_layout[0]);
        frame.render_widget(splash_para, layout[0]);
        frame.render_stateful_widget(table, layout[0], &mut self.result_table_state);

        render_sparkline_from(
            frame,
            layout[1],
            &self.answered_questions,
            Direction::Vertical,
        );
    }
    // fn draw_end_splash()

    fn draw(&self, frame: &mut Frame) {
        let layout: std::rc::Rc<[Rect]> =
            Layout::new(Direction::Vertical, [Constraint::Percentage(100)]).split(frame.area());
        frame.render_widget(self, layout[0]);
        // frame.render_widget(Block::bordered(), layout[1]);
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let debug_times = generate_random_durations(
            Duration::from_secs(self.gameconfig.timer as u64),
            self.gameconfig.debug_questions,
        );
        let mut debug_index = 0;

        while !self.exit {
            // determine which screen to draw based on the GameState
            match self.gamestate {
                GameState::Setup => terminal.draw(|frame| self.draw_splash(frame))?,
                GameState::Inprogress => terminal.draw(|frame| self.draw(frame))?,
                GameState::EndingSplash => {
                    terminal.draw(|frame: &mut Frame<'_>| self.draw_end_splash(frame))?
                }
                GameState::HistorySplash => {
                    terminal.draw(|frame| self.draw_history_splash(frame))?
                }
                GameState::SettingsSpash => todo!(),
            };

            self.current_time = Local::now();

            if self.gameconfig.debug
                && debug_index < debug_times.len()
                && self.gamestate != GameState::EndingSplash
            {
                if self.gamestate == GameState::Setup {
                    self.gamestate = GameState::Inprogress;
                }
                thread::sleep(debug_times[debug_index]);
                self.input = self.current_question.answer.to_string();
                self.score += 1;
                self.input.clear();
                self.current_question.question_answer = Some(Local::now());
                self.answered_questions.push(self.current_question);

                self.current_question = MathQuestion::generate_new_question(&self.gameconfig.qr);

                debug_index += 1;
            } else if self.gamestate == GameState::Inprogress && debug_index == debug_times.len() {
                self.gamestate = GameState::EndingSplash;
            } else {
                poll(Duration::from_millis(10))?;
                {
                    //pings crossterm for input every 10ms
                    self.handle_events()?;
                }
            }

            // game over on timeout
            if self.gamestate == GameState::Inprogress
                && (self.current_time - self.start_time).num_seconds()
                    > self.gameconfig.timer.try_into().unwrap()
            {
                if self.answered_questions.last().is_some() {
                    // self.answered_questions.last().unwrap().question_answer = Some(Instant::now());
                }
                self.current_question.question_answer = Some(Local::now());
                self.answered_questions.push(self.current_question);
                //this will panic if too long. TODO fix
                self.gamestate = GameState::EndingSplash;
                self.game_history.add_game_result(GameRecord {
                    game_intant: Utc::now(),
                    score: self.score,
                    answers: self.answered_questions.clone(),
                });
                let _ = self.game_history.save();
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match self.gamestate {
                    GameState::Setup => self.handle_key_event_splash(key_event),
                    GameState::Inprogress => self.handle_key_event_game(key_event),
                    GameState::EndingSplash => self.handle_end_event_splash(key_event),
                    GameState::HistorySplash => self.handle_key_event_history(key_event),
                    GameState::SettingsSpash => self.handle_key_event_game(key_event),
                }
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_end_event_splash(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.result_table_state.select_previous(),
            KeyCode::Down => self.result_table_state.select_next(),
            _ => {}
        }
    }

    fn handle_key_event_history(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.history_table_state.select_previous(),
            KeyCode::Down => self.history_table_state.select_next(),
            _ => {}
        }
    }

    fn handle_key_event_splash(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('s') => {
                self.gamestate = GameState::Inprogress;
                self.start_time = Local::now();
                self.current_question = MathQuestion::generate_new_question(&self.gameconfig.qr)
            }
            KeyCode::Char('h') => {
                self.gamestate = GameState::HistorySplash;
            }

            KeyCode::Delete => {
                self.input.pop();
            }

            _ => {}
        }

        let solved = self.input.parse::<i32>() == Ok(self.current_question.answer);
        // correct answer
        if solved {
            self.score += 1;
            self.input.clear();
            self.current_question = MathQuestion::generate_new_question(&self.gameconfig.qr);
        }
    }

    fn handle_key_event_game(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Delete => {
                self.input.pop();
            }

            _ => self.input.push_str(&key_event.code.to_string()),
        }

        let solved = self.input.parse::<i32>() == Ok(self.current_question.answer);
        if solved {
            self.score += 1;
            self.input.clear();
            self.current_question.question_answer = Some(Local::now());
            self.answered_questions.push(self.current_question);

            self.current_question = MathQuestion::generate_new_question(&self.gameconfig.qr);
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = MathGame::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
