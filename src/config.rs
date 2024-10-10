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
            timer: 120,
            qr: QuestionRanges::default(),
            debug: false,
            debug_questions: 72,
        }
    }
}
#[derive(Debug)]
pub struct QuestionRanges {
    pub add_lower: i32,
    pub add_upper: i32,
    pub mult_rhs_lower: i32,
    pub mult_rhs_upper: i32,
    pub mult_lhs_lower: i32,
    pub mult_lhs_upper: i32,
}

impl Default for QuestionRanges {
    fn default() -> Self {
        Self {
            add_lower: 2,
            add_upper: 100,
            mult_lhs_lower: 2,
            mult_lhs_upper: 12,
            mult_rhs_lower: 2,
            mult_rhs_upper: 100,
        }
    }
}
