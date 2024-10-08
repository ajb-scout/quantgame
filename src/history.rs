use ratatui::layout::Constraint;
use ratatui::style::{Color, Style};
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use chrono::serde::ts_nanoseconds;
use ratatui::widgets::*;
use chrono::{DateTime, Local, Utc};

use crate::{MathAnswer, MathQuestion};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameHistory {
    path: String,
    pub history: Vec<GameRecord>,
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameRecord {
    #[serde(with = "ts_nanoseconds")]
    pub game_intant: DateTime<Utc>,
    pub score: i32, 
    pub answers: Vec<MathAnswer>
}

impl Default for GameHistory {
    fn default() -> Self {
        Self { path: Default::default(), history: Default::default() }
    }
}



impl GameHistory {
    // Load or create the game history from a specified path
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<GameHistory> {
        let path_str = path.as_ref().to_string_lossy().into_owned();
        
        if Path::new(&path_str).exists() {
            // If the file exists, read the content
            let mut file = File::open(&path_str)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            // Deserialize the JSON content into a GameHistory struct
            let mut history: GameHistory = serde_json::from_str(&contents)?;
            history.path = path_str; // Update the path

            Ok(history)
        } else {
            // If the file does not exist, create a new GameHistory with an empty history
            let new_history = GameHistory {
                path: path_str,
                history: Vec::new(),
            };
            // Write the default state to the file
            let json = serde_json::to_string(&new_history)?;
            let mut file = File::create(&new_history.path)?;
            file.write_all(json.as_bytes())?;
            Ok(new_history)
        }
    }

    // Add a new game result to the history
    pub fn add_game_result(&mut self, result: GameRecord) {
        self.history.push(result);
    }

    // Save the game history to a file
    pub fn save(&self) -> io::Result<()> {
        let json = serde_json::to_string(self)?;
        let mut file = File::create(&self.path)?;
        file.write_all(json.as_bytes())
    }
}

pub fn render_table_from_history(hs: &GameHistory) -> Table {
    let header = ["#", "Date", "Score"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(2);


    let mut rows: Vec<Row> = vec![];
    for (x, i) in hs.history.iter().enumerate() {
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
    return table;
}
