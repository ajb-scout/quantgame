use chrono::{DateTime, Local, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, TableState, Widget,
    },
    Frame,
};

use crate::{
    config::{GameConfiguration, QuestionRanges},
    history::{GameHistory, GameRecord},
    util::{self, Sign},
};

#[derive(Debug, PartialEq)]
pub enum GameState {
    Setup,
    Inprogress,
    EndingSplash,
    HistorySplash,
    SettingsSpash,
}

#[derive(Debug)]
pub struct MathGame {
    pub current_question: MathQuestion,
    // game_is_started: bool,
    pub exit: bool,
    pub input: String,
    pub score: i32,
    pub start_time: DateTime<Local>,
    pub current_time: DateTime<Local>,
    pub questions: Vec<MathQuestion>,
    pub answers: Vec<MathAnswer>,
    pub gamestate: GameState,
    pub gameconfig: GameConfiguration,
    pub result_table_state: TableState,
    pub game_history: GameHistory,
    pub history_table_state: TableState,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MathQuestion {
    pub lhs: i32,
    pub rhs: i32,
    pub answer: i32,
    pub sign: Sign,
    pub question_start: DateTime<Local>,
    pub question_answer: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathAnswer {
    pub q: MathQuestion,
    pub string_representation: String,
    pub duration_s: i64,
    pub duration_m: i64,
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
            questions: vec![],
            answers: vec![],
            gamestate: GameState::Setup,
            gameconfig: config,
            result_table_state: TableState::default().with_selected(0),
            game_history: GameHistory::new("results.json").unwrap_or_default(),
            history_table_state: TableState::default().with_selected(0),
        }
    }
}

impl MathGame {
    pub fn get_elapsed_time_seconds(&self) -> i32 {
        return (Local::now() - self.start_time).num_seconds() as i32;
    }
    pub fn handle_game_start(&mut self) {
        self.score = 0;
        self.answers = vec![];
        self.questions = vec![];
        self.current_question = MathQuestion::generate_new_question(&self.gameconfig.qr);
        self.current_time = Local::now();
        self.start_time = Local::now();
        self.gamestate = GameState::Inprogress;
    }

    pub fn handle_return_to_splash(&mut self){
        self.gamestate = GameState::Setup;
        self.result_table_state.select_first();
    }

    pub fn handle_game_end(&mut self, save: bool) {
        self.current_question.question_answer = Some(Local::now());
        self.questions.push(self.current_question);
        self.answers = self
            .questions
            .iter()
            .map(|f| f.generate_math_answer())
            .collect();

        //this will panic if too long. TODO fix
        self.game_history.add_game_result(GameRecord {
            game_intant: Utc::now(),
            score: self.score,
            answers: self.answers.clone(),
        });
        if save {
            let saved = self.game_history.save();
            match saved {
                Ok(_) => {},
                Err(e) => panic!("{}", e),
            }
        }
        self.gamestate = GameState::EndingSplash;
    }

    pub fn handle_game_restart(&mut self) {
        let _ = &self.handle_game_start();
    }

    pub fn draw(&self, frame: &mut Frame) {
        let layout: std::rc::Rc<[Rect]> =
            Layout::new(Direction::Vertical, [Constraint::Percentage(100)]).split(frame.area());
        frame.render_widget(self, layout[0]);
        // frame.render_widget(Block::bordered(), layout[1]);
    }

    // /// runs the application's main loop until the user quits
    // pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    //     let debug_times = crate::util::generate_random_durations(
    //         Duration::from_secs(self.gameconfig.timer as u64),
    //         self.gameconfig.debug_questions,
    //     );
    //     let mut debug_index = 0;

    //     while !self.exit {
    //         // determine which screen to draw based on the GameState
    //         match self.gamestate {
    //             GameState::Setup => terminal.draw(|frame| render_game_splash(frame, self))?,
    //             GameState::Inprogress => terminal.draw(|frame| self.draw(frame))?,
    //             GameState::EndingSplash => {
    //                 terminal.draw(|frame: &mut Frame<'_>| render_end_splash(frame, self))?
    //             }
    //             GameState::HistorySplash => {
    //                 terminal.draw(|frame| crate::renderers::render_history_splash(frame, self))?
    //             }
    //             GameState::SettingsSpash => todo!(),
    //         };

    //         self.current_time = Local::now();

    //         if self.gameconfig.debug
    //             && debug_index < debug_times.len()
    //             && self.gamestate != GameState::EndingSplash
    //         {
    //             if self.gamestate == GameState::Setup {
    //                 self.gamestate = GameState::Inprogress;
    //             }
    //             thread::sleep(debug_times[debug_index]);
    //             self.input = self.current_question.answer.to_string();
    //             self.score += 1;
    //             self.input.clear();
    //             self.current_question.question_answer = Some(Local::now());
    //             self.questions.push(self.current_question);

    //             self.current_question = MathQuestion::generate_new_question(&self.gameconfig.qr);

    //             debug_index += 1;
    //         } else if self.gamestate == GameState::Inprogress && debug_index == debug_times.len() {
    //             self.gamestate = GameState::EndingSplash;
    //         } else {
    //             poll(Duration::from_millis(10))?;
    //             {
    //                 //pings crossterm for input every 10ms
    //                 let _ = crate::event_handlers::handle_events(self);
    //             }
    //         }

    //         // game over on timeout
    //         if self.gamestate == GameState::Inprogress
    //             && (self.current_time - self.start_time).num_seconds() as i32
    //                 > self.gameconfig.timer
    //         {
    //             self.handle_game_end(true);
    //         }
    //     }
    //     Ok(())
    // }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &MathGame {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let score = Title::from(Line::from(vec![
            " Score:  ".into(),
            self.score.to_string().bold(),
            "  Elapsed:  ".into(),
            self.get_elapsed_time_seconds().to_string().bold(),
            " ".into(),
        ]));

            let instructions = Title::from(Line::from(vec![
                " Reset ".into(),
                "<R>".blue().bold(),
                " Quit ".into(),
                "<Q>".blue().bold(),
                " Return to Start ".into(),
                "<D> ".blue().bold(),
                " End ".into(),
                "<E> ".blue().bold(),
            ]));

        let block: Block<'_> = Block::bordered()
            .title(score.alignment(Alignment::Center).position(Position::Top))
            .title(instructions.alignment(Alignment::Center).position(Position::Bottom))

            .border_set(border::DOUBLE);
        let input_line = Span::from(self.input.clone().white());

        let counter_text = Text::from(vec![
            Line::from(vec![format!("Question {}: ", self.score + 1).yellow()]),
            Line::from(vec![
                self.current_question.lhs.to_string().into(),
                " ".into(),
                util::match_sign(&self.current_question.sign)
                    .to_string()
                    .into(),
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

// class representing an individual math question
impl MathQuestion {

    //generates a math answer, used to stop recomputing each UI tick
    fn generate_math_answer(self) -> MathAnswer {
        let srep = format!(
            "{:<3} {} {:<3}",
            self.lhs.to_string(),
            self.sign.to_string(),
            self.rhs.to_string()
        );
        let duration_s =
            (self.question_answer.unwrap_or(Local::now()) - self.question_start).num_seconds();
        let duration_m =
            (self.question_answer.unwrap_or(Local::now()) - self.question_start).num_milliseconds();
        return MathAnswer {
            q: self,
            string_representation: srep,
            duration_s,
            duration_m,
        };
    }

    //generates the LHS and RHS values for a question given a question range
    fn generate_lhs_rhs(qr: &QuestionRanges, sign: &Sign) -> (i32, i32) {
        let mut rng = rand::thread_rng();

        return match sign {
            Sign::Multiply => (
                rng.gen_range(qr.mult_lhs_lower..qr.mult_lhs_upper),
                rng.gen_range(qr.mult_rhs_lower..qr.mult_rhs_upper),
            ),
            Sign::Add => (
                rng.gen_range(qr.add_lower..qr.add_upper),
                rng.gen_range(qr.add_lower..qr.add_upper),
            ),
            Sign::Subtract => {
                let lhs = rng.gen_range(qr.add_lower..qr.add_upper);
                let rhs = rng.gen_range(qr.add_lower..qr.add_upper);
                // always have a positive answer
                if lhs > rhs {
                    return (lhs, rhs);
                }
                (rhs, lhs)
            }
            Sign::Divide => {
                let lhs = rng.gen_range(qr.mult_lhs_lower..qr.mult_lhs_upper);
                let rhs = rng.gen_range(qr.mult_rhs_lower..qr.mult_rhs_upper); 
                // get divide from mult to ensure round number answer
                let ans = lhs * rhs;
                (ans, lhs)
            }
        };
    }

    //randomly generate a new question
    pub fn generate_new_question(qr: &QuestionRanges) -> MathQuestion {
        let sign = match rand::thread_rng().gen_range(1..5) {
            1 => Sign::Add,
            2 => Sign::Subtract,
            3 => Sign::Multiply,
            4 => Sign::Divide,
            _ => panic!("Shouldn't be able to generate this"),
        };

        let lhs_rhs = Self::generate_lhs_rhs(qr, &sign);
        let answer: i32 = util::apply_sign(&sign, lhs_rhs.0, lhs_rhs.1);
        
        let new_question = MathQuestion {
            lhs: lhs_rhs.0,
            rhs: lhs_rhs.1,
            answer,
            sign,
            question_start: Local::now(),
            question_answer: Option::None,
        };
        return new_question;
    }
}
