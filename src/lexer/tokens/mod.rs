pub(crate) mod function_token;
pub mod binary_operator_token;

use std::any::Any;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
#[derive(Eq, PartialEq, Clone)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    UnaryOperator,
    BinaryOperator,
    Variable,
    Name,
    Number,
    Comma,
    EndOfExpression,
    Function,
    Equal,
}

pub trait TokenTrait {
    fn get_token_type(&self) -> TokenType;
    fn get_value(&self) -> String;

    fn print(&self) {
        print!("[{:?}] => {:?}, ", self.get_token_type(), self.get_value());
    }

    fn set_token_type(&mut self, token_type: TokenType);

    fn as_any(&self) -> &dyn Any;
}

impl Debug for dyn TokenTrait {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] => {}", self.get_token_type(), self.get_value())

    }
}

pub struct Token {
    token_type: TokenType,
    value: String
}

impl Token {
    pub(crate) fn new(token_type: TokenType, value: String) -> Self {
        return Self {token_type, value};
    }
}

impl TokenTrait for Token {
    fn get_token_type(&self) -> TokenType {
        return self.token_type.clone();
    }

    fn get_value(&self) -> String {
        return self.value.clone();
    }

    fn set_token_type(&mut self, token_type: TokenType) {
        self.token_type = token_type;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for Token{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] => {}", self.token_type, self.value)
    }
}
