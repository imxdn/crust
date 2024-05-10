use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Token {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EXPONENT,
    INTEGER { val: i64 },
    FLOAT { val: f64 },
}
