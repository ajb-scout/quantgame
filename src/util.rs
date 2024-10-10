use std::{fmt::Display, time::Duration};

use rand_distr::{Distribution, Normal};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::game::{MathAnswer, MathGame};

pub const ASCII_TITLE: [&str; 5] = [
    "   ____                   _     ___                     ",
    r"  /___ \_   _  __ _ _ __ | |_  / _ \__ _ _ __ ___   ___ ",
    r" //  / / | | |/ _` | '_ \| __|/ /_\/ _` | '_ ` _ \ / _ \",
    r"/ \_/ /| |_| | (_| | | | | |_/ /_\\ (_| | | | | | |  __/",
    r"\___,_\ \__,_|\__,_|_| |_|\__\____/\__,_|_| |_| |_|\___|",
];

pub fn apply_sign(sign: &Sign, lhs: i32, rhs: i32) -> i32 {
    return match sign {
        Sign::Multiply => lhs * rhs,
        Sign::Add => lhs + rhs,
        Sign::Subtract => lhs - rhs,
        Sign::Divide => lhs / rhs,
    };
}

pub fn match_sign(sign: &Sign) -> char {
    match sign {
        Sign::Multiply => 'x',
        Sign::Add => '+',
        Sign::Subtract => '-',
        Sign::Divide => '/',
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Sign {
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

pub(crate) fn create_gradient(colors: &[f32]) -> Vec<Color> {
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

pub fn generate_random_durations(total_duration: Duration, count: i32) -> Vec<Duration> {
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

pub fn get_target_answers(game: &MathGame) -> &Vec<MathAnswer> {
    match game.gamestate {
        crate::game::GameState::HistorySplash => {
            &game.game_history.history[game
                .history_table_state
                .selected()
                .unwrap_or_default()
                .min(game.game_history.history.len() - 1)]
            .answers
        }
        _ => &game.answers,
    }
}
