use std::{collections::HashMap, f32::consts::E, time::Instant};

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use rand::Rng;

use std::io;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

use std::thread;
use std::time::Duration;

struct Timer<F> {
    delay: Duration,
    action: F,
}

impl<F> Timer<F>
where
    F: FnOnce() + Send + Sync + 'static,
{
    fn new(delay: Duration, action: F) -> Self {
        Timer { delay, action }
    }

    fn start(self) {
        thread::spawn(move || {
            thread::sleep(self.delay);
            (self.action)();
        });
    }
}


fn binomial(increment: i32, mut remaining: i32) -> Vec<i32>{
    let mut oldvar = vec![increment];
    while remaining > 0 {
        let mut newvar: Vec<i32> = vec![];

        for i in oldvar{
            let plusval = i + increment*2;
            let minval: i32 = i - increment/2;
            newvar.push(plusval);
            newvar.push(minval);
        }
        // println!("{:?}", newvar);
        oldvar = newvar;

        remaining -= 1;
    }
    return oldvar;

}
    
#[derive(Debug)]
pub struct BinomialPut {
    t: f32,
    T: u8,
    d: f32,
    u: f32, 
    r: f32,
    vol: f32,
    strike: f32, 
    price: f32,
    exit: bool,

}

#[derive(Debug)]
pub struct MathGame {
    current_question: MathQuestion,
    game_is_started: bool,
    exit: bool,
    input: String,
    score: i32,
    add_lower: i32,
    add_upper: i32,
    mult_lower: i32,
    mult_upper: i32,
    start_time: Instant, 
    current_time: Instant
}

impl Default for MathGame {
    fn default() -> Self {
        Self { current_question: Default::default(), game_is_started: Default::default(), exit: Default::default(), input: Default::default(), score: Default::default(), add_lower: 2, add_upper: 100, mult_lower: 2, mult_upper: 12, start_time: Instant::now(), current_time: Instant::now() }
    }
}
#[derive(Debug)]
pub struct MathQuestion{
    lhs: i32,
    rhs: i32,
    answer: i32,
    sign: Sign
}

impl Default for MathQuestion {
    fn default() -> Self {
        Self { lhs: 2, rhs: 3, answer: 5, sign: Sign::Add }
    }
}

#[derive(Debug)]
enum Sign {
    Multiply,
    Add,
    Subtract,
    Divide
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
            " ".into(),
            " Elapsed:  ".into(),
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
        let block2 = Block::bordered()
            .title(Title::from(""))
            
            .border_set(border::ROUNDED);
            // .border_style(Style::new().blue());
        let solved = self.input.parse::<i32>() == Ok(self.current_question.answer);
        let mut input_line = Span::from(self.input.clone().white());    
        if solved{
            input_line = Span::from(self.input.clone().green());
    
        } 


        let counter_text = Text::from(vec![
            Line::from(vec!["Question 1: ".yellow()]),
            Line::from(vec![
                self.current_question.lhs.to_string().into(), 
                " ".into(),
                match_sign(&self.current_question.sign).to_string().into(),
                " ".into(),
                self.current_question.rhs.to_string().into(),
                " = ".into(),
                self.current_question.answer.to_string().into(),

                input_line,

                ]),

            ]);


        Paragraph::new(counter_text)
            // .centered()
            
            .alignment(Alignment::Center)
            .block(block)
            // .block(block2)
            .render(area, buf);
}}

impl  MathGame {
    fn generate_lhs_rhs(self: &mut MathGame, sign: &Sign) -> (i32, i32){
        return match sign {
            Sign::Multiply => {(rand::thread_rng().gen_range(self.mult_lower..self.mult_upper), rand::thread_rng().gen_range(self.mult_lower..self.mult_upper))},
            Sign::Add => {(rand::thread_rng().gen_range(self.add_lower..self.add_upper), rand::thread_rng().gen_range(self.add_lower..self.add_upper))},
            Sign::Subtract => {
                let lhs = rand::thread_rng().gen_range(self.add_lower..self.add_upper);
                let rhs = rand::thread_rng().gen_range(self.add_lower..self.add_upper);
                if (lhs>rhs){
                    return (lhs,rhs)
                }
                (rhs, lhs)

            },
            Sign::Divide => {(rand::thread_rng().gen_range(self.mult_lower..self.mult_upper), rand::thread_rng().gen_range(self.mult_lower..self.mult_upper))},
        }
    }

    fn generate_new_question(self: &mut MathGame) -> MathQuestion {
        let sign =  match rand::thread_rng().gen_range(1..5) {
            1 => Sign::Add,
            2 => Sign::Subtract,
            3 => Sign::Multiply,
            4 => Sign::Divide,

            _ => panic!("Shouldn't be able to generate this")

        };

        let lhs_rhs = Self::generate_lhs_rhs(self, &sign);

        let lhs = rand::thread_rng().gen_range(0..12);
        let rhs = rand::thread_rng().gen_range(0..12);
        // let sign = Sign::Add; 
        let answer = apply_sign(&sign, lhs_rhs.0, lhs_rhs.1);

        let new_question = MathQuestion { lhs: lhs_rhs.0, rhs: lhs_rhs.1, answer: answer, sign: sign};
        return new_question


    }
    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Length(30), Constraint::Min(10)],
        )
        .split(Rect::new(0, 0, 30, 20));
   
           frame.render_widget(self, layout[0]);
           frame.render_widget(Block::bordered(), layout[1]);
   
       }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {

            terminal.draw(|frame| self.draw(frame))?;
            self.current_time = Instant::now(); 
            // if let Event::Key(key) = event::read()? {
                if poll(Duration::from_millis(10))? {
                    // It's guaranteed that `read` won't block, because `poll` returned
                    // `Ok(true)`.
                     self.handle_events()?;
                }
            // self.current_time.elapsed()
        }
        Ok(())
    }
    
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Backspace => {self.input.pop();}
            KeyCode::Delete => {self.input.pop();}

            // KeyCode::Left => self.decrement_counter(),
            // KeyCode::Right => self.increment_counter(),
            _ => {self.input.push_str(&key_event.code.to_string())}
        }

        let solved = self.input.parse::<i32>() == Ok(self.current_question.answer);
        if solved{
            self.score += 1;
            self.input.clear();
            

            self.current_question = Self::generate_new_question(self)
            
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
