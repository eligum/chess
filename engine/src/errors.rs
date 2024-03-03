//! Custom error types for this crate.

pub enum Engine {
    IoError(String),
    ParseError(String),
}
