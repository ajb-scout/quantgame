#[derive(Debug)]
pub struct GameConfiguration {
    pub endless: bool,
    pub timer: i32, 
    pub qr: QuestionRanges,
    pub debug: bool,
    pub debug_questions: i32,
}

impl Default for GameConfiguration {
    fn default() -> Self {
        Self { 
            endless: false, 
            timer: 5, qr: 
            QuestionRanges::default(),
            debug: false,
            debug_questions: 72
        }
    }
}
#[derive(Debug)]
pub struct QuestionRanges {
    pub add_lower: i32,
    pub add_upper: i32,
    pub mult_lower: i32,
    pub mult_upper: i32,
}

impl Default for QuestionRanges {
    fn default() -> Self {
        Self {
            add_lower: 2,
            add_upper: 100,
            mult_lower: 2,
            mult_upper: 12,
 }
    }
}
