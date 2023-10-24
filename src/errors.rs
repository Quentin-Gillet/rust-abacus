use std::process::exit;
use crate::errors::ErrorType::{InvalidOperation, SyntaxError, UnboundVariable, UnexpectedToken};

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    SyntaxError,
    UnexpectedToken,
    InvalidOperation,
    UnboundVariable,
}

struct Errors {
    errors: Vec<(ErrorType, String, i8)>,
}

impl Default for Errors {
    fn default() -> Self {
        Self {
            errors: vec![
                (SyntaxError, "Syntax error.".to_string(), 2),
                (UnexpectedToken, "Unexpected token.".to_string(), 2),
                (InvalidOperation, "Invalid operation.".to_string(), 3),
                (UnboundVariable, "Unbound variable.".to_string(), 3),
            ]
        }
    }
}

pub struct Error;
impl Error {
    pub fn throw<T>(error_type: ErrorType) -> T {

        let errors = Errors::default();

        let error: (ErrorType, String, i8) = errors.errors.into_iter().find(|(error_type_, _, _)| *error_type_ == error_type).unwrap();
        let (_, error_message, error_code) = error;
        eprintln!("{}", error_message);
        exit(error_code as i32);
    }
}