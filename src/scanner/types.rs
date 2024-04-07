#[derive(Debug)]
pub enum Token {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EXPONENT,
    INTEGER {val: i64},
    FLOAT {val: f64}
}
