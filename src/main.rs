mod config;
use std::time::Instant;
use config::QuestionRanges;
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::Rng;
use crate::config::GameConfiguration;
use std::io;

use ratatui::{
    buffer::Buffer, crossterm, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::Stylize, symbols::border, text::{Line, Span, Text, ToSpan}, widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget, Wrap,
    }, DefaultTerminal, Frame
};

use std::time::Duration;


const ascii_title: [&str;5] = [
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
    start_time: Instant, 
    current_time: Instant,
    gamestate: GameState,
    gameconfig: GameConfiguration
}

impl Default for MathGame {
    fn default() -> Self {
        let config = GameConfiguration::default();
        Self { 
            current_question: MathQuestion::generate_new_question(&config.qr), 
            // game_is_started: Default::default(), 
            exit: Default::default(), 
            input: Default::default(), 
            score: Default::default(),
            start_time: Instant::now(),
            current_time: Instant::now(),
            gamestate: GameState::Setup,
            gameconfig: config
        }
    }
}
#[derive(Debug)]
pub struct MathQuestion{
    lhs: i32,
    rhs: i32,
    answer: i32,
    sign: Sign,
}

impl MathQuestion {

    fn generate_lhs_rhs(qr: &QuestionRanges, sign: &Sign) -> (i32, i32){
        return match sign {
        Sign::Multiply => {(rand::thread_rng().gen_range(qr.mult_lower..qr.mult_upper), rand::thread_rng().gen_range(qr.mult_lower..qr.mult_upper))},
        Sign::Add => {(rand::thread_rng().gen_range(qr.add_lower..qr.add_upper), rand::thread_rng().gen_range(qr.add_lower..qr.add_upper))},
        Sign::Subtract => {
            let lhs = rand::thread_rng().gen_range(qr.add_lower..qr.add_upper);
            let rhs = rand::thread_rng().gen_range(qr.add_lower..qr.add_upper);
            if lhs>rhs{
                return (lhs,rhs)
            }
            (rhs, lhs)

        },
        Sign::Divide => {(rand::thread_rng().gen_range(qr.mult_lower..qr.mult_upper), rand::thread_rng().gen_range(qr.mult_lower..qr.mult_upper))},
    }
    }

pub fn generate_new_question(qr: &QuestionRanges) -> MathQuestion {
    let sign =  match rand::thread_rng().gen_range(1..5) {
        1 => Sign::Add,
        2 => Sign::Subtract,
        3 => Sign::Multiply,
        4 => Sign::Divide,
        _ => panic!("Shouldn't be able to generate this")
    };

    let lhs_rhs = Self::generate_lhs_rhs(qr, &sign);
    let answer: i32 = apply_sign(&sign, lhs_rhs.0, lhs_rhs.1);
    let new_question = MathQuestion { lhs: lhs_rhs.0, rhs: lhs_rhs.1, answer: answer, sign: sign};
    return new_question
    }
}

#[derive(Debug)]
enum Sign {
    Multiply,
    Add,
    Subtract,
    Divide
}
#[derive(Debug, PartialEq)]
enum GameState {
    Setup,
    Inprogress,
    EndingSplash
}

fn match_sign(sign: &Sign) -> char {
    match sign {
        Sign::Multiply => 'x',
        Sign::Add => '+',
        Sign::Subtract => '-',
        Sign::Divide => '/',
    }
}

fn apply_sign(sign: &Sign, lhs: i32, rhs: i32) -> i32{
    return match sign {
        Sign::Multiply => lhs*rhs,
        Sign::Add => lhs+rhs,
        Sign::Subtract => lhs-rhs,
        Sign::Divide => lhs/rhs,
    }
}

impl Widget for &MathGame {

    fn render(self, area: Rect, buf: &mut Buffer){
        let title = Title::from("Quant Game".bold());

        let score = Title::from(Line::from(vec![
            " Score:  ".into(),
            self.score.to_string().bold(),
            "  Elapsed:  ".into(),
            self.current_time.duration_since(self.start_time).as_secs().to_string().bold(),
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

impl MathGame {

    fn draw_splash(&self, frame: &mut Frame){
        let outer_layout = Layout::new(
            Direction::Vertical, vec![Constraint::Percentage(10), Constraint::Percentage(70), Constraint::Percentage(20)]).split(frame.area());

        // build the ascii string from the title 
        let mut title_vec = vec![];
        for i in ascii_title.iter().copied() {
            title_vec.push(Line::from(i));
        }
        
        // build text objects
        let splash_text = Text::from(title_vec).alignment(Alignment::Left);
        let options_text = (Span::from("S").underlined() + Span::from("tart")) + (Span::from("Q").underlined() + Span::from("uit")) + (Span::from("S") + Span::from("e").underlined() + Span::from("ttings"));
        
        // build splash para
        let splash_para = Paragraph::new(splash_text).alignment(Alignment::Center).wrap(Wrap {trim: false});
        let options_para = Paragraph::new(options_text).alignment(Alignment::Center).wrap(Wrap {trim: false});

        // render the widgets
        frame.render_widget(Block::bordered().border_set(border::ROUNDED), frame.area());
        frame.render_widget(splash_para, outer_layout[1]);
        frame.render_widget(options_para, outer_layout[2]);
    }

    fn draw(&self, frame: &mut Frame) {
        let layout: std::rc::Rc<[Rect]> = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(frame.area());
        frame.render_widget(self, layout[0]);
        frame.render_widget(Block::bordered(), layout[1]);
       }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {

            match self.gamestate {
                GameState::Setup => terminal.draw(|frame| self.draw_splash(frame))?,
                GameState::Inprogress => terminal.draw(|frame| self.draw(frame))?,
                GameState::EndingSplash => todo!(),
            };

            self.current_time = Instant::now(); 
                if poll(Duration::from_millis(10))? { //pings crossterm for input every 10ms
                     self.handle_events()?;
            }

            if self.gamestate == GameState::Inprogress && self.current_time.duration_since(self.start_time).as_secs() > self.gameconfig.timer.try_into().unwrap() { //this will panic if too long. TODO fix
                self.gamestate = GameState::EndingSplash;
            }
        }
        Ok(())
    }
    
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match self.gamestate{
                    GameState::Setup => self.handle_key_event_splash(key_event),
                    GameState::Inprogress => self.handle_key_event_game(key_event),
                    GameState::EndingSplash => todo!(),
                }
        
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event_splash(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('s') => {self.gamestate = GameState::Inprogress; self.start_time = Instant::now()},
            KeyCode::Delete => {self.input.pop();}

            _ => {self.input.push_str(&key_event.code.to_string())}
        }

        let solved = self.input.parse::<i32>() == Ok(self.current_question.answer);
        if solved{
            self.score += 1;
            self.input.clear();
            self.current_question = MathQuestion::generate_new_question(&self.gameconfig.qr);
        }
    }

    fn handle_key_event_game(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Backspace => {self.input.pop();}
            KeyCode::Delete => {self.input.pop();}

            _ => {self.input.push_str(&key_event.code.to_string())}
        }

        let solved = self.input.parse::<i32>() == Ok(self.current_question.answer);
        if solved{
            self.score += 1;
            self.input.clear();
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
