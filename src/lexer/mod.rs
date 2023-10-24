pub mod tokens;
pub(crate) mod shunting_yard;

use crate::errors::{Error, ErrorType};
use crate::lexer::tokens::{Token, TokenType, TokenTrait};
use crate::lexer::tokens::function_token::{DefinedFunction, TokenFunction};
use crate::lexer::tokens::binary_operator_token::TokenOperator;

pub struct Lexer {
    input: String,
    position: usize,
    current_char: char,
    pub tokens: Vec<Box<dyn TokenTrait>>
}

impl Lexer {
    pub(crate) fn new(input: &str) -> Lexer {
        let string_input = input.to_string();
        Lexer {
            input: string_input,
            position: 0,
            current_char: input.chars().nth(0).unwrap_or(' '),
            tokens: vec![]
        }
    }

    pub(crate) fn process(&mut self) {
        while self.position < self.input.len() {
            self.make_token();
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.position > self.input.len() - 1 {
            self.current_char = ' ';
            return;
        }
        self.current_char = self.input.chars().nth(self.position).unwrap_or(' ');
    }

    fn make_token(&mut self) {
        if self.current_char.is_numeric() {
            self.make_numer_token();
            return;
        }
        else if TokenOperator::get_operator_value(&self.current_char.to_string()).is_some() {
            self.make_token_operator();
        }
        else if self.current_char == '(' {
            let last_token = self.tokens.last();
            match last_token {
                Some(token) => {
                    if token.get_token_type() == TokenType::Variable ||
                        token.get_token_type() == TokenType::Number ||
                        token.get_token_type() == TokenType::RightParenthesis {
                        self.tokens.push(Box::new(TokenOperator::new(
                            TokenType::BinaryOperator,
                            "*".to_string(),
                        )));
                    }
                }
                None => {}
            }
            self.tokens.push(Box::new(Token::new(
                TokenType::LeftParenthesis,
                self.current_char.to_string()
            )));
        }
        else if self.current_char == ')' {
            self.tokens.push(Box::new(Token::new(
                TokenType::RightParenthesis,
                self.current_char.to_string()
            )));
        }
        else if self.current_char == ',' {
            let last_token = self.tokens.last();
            match last_token {
                Some(token) => {
                    if token.get_token_type() != TokenType::Number &&
                        token.get_token_type() != TokenType::RightParenthesis &&
                        token.get_token_type() != TokenType::Variable {
                        Error::throw::<()>(ErrorType::SyntaxError);
                    }
                }
                None => Error::throw::<()>(ErrorType::SyntaxError)
            }
            self.tokens.push(Box::new(Token::new(
                TokenType::Comma,
                self.current_char.to_string()
            )));
        }
        else if self.current_char == ' ' || self.current_char == '\t' ||
                self.current_char == '\n' || self.current_char == ';' {
            if self.current_char == ';' {
                self.tokens.push(Box::new(Token::new(
                    TokenType::EndOfExpression,
                    self.current_char.to_string()
                )));
            }
            self.advance();
            return;
        }
        else if self.current_char == '=' {
            if self.tokens.len() == 0 {
                Error::throw::<()>(ErrorType::SyntaxError);
            }
            if let Some(last_token) = self.tokens.last_mut() {
                if last_token.get_token_type() == TokenType::Variable {
                    last_token.set_token_type(TokenType::Name);
                }
                else {
                    Error::throw::<()>(ErrorType::SyntaxError);
                }
            }
        }
        else if self.current_char.is_alphanumeric() || self.current_char == '_' {
            self.make_name_token();
            return;
        }
        else {
            Error::throw::<()>(ErrorType::UnexpectedToken);
        }

        self.advance();
    }

    fn make_numer_token(&mut self) {
        let mut number: String = String::new();
        let mut dot_count: usize = 0;

        while self.current_char.is_numeric() || self.current_char == '.' {
            if self.current_char == '.' {
                dot_count += 1;
                if dot_count > 1 {
                    Error::throw::<()>(ErrorType::SyntaxError);
                }
            }
            number.push(self.current_char);
            self.advance();
        }

        self.tokens.push(Box::new(Token::new(
            TokenType::Number,
            number
        )));
    }

    fn make_name_token(&mut self) {
        let mut name: String = String::new();

        while self.current_char.is_alphanumeric() || self.current_char == '_' {
            name.push(self.current_char);
            self.advance();
        }

        if DefinedFunction::is_defined_function(&name).is_some() {
            self.tokens.push(Box::new(TokenFunction::new(
                TokenType::Function,
                name
            )));
            return;
        }

        let mut token_type = TokenType::Variable;
        if let Some(last_token) = self.tokens.last() {
            match last_token.get_token_type() {
                TokenType::Number => {
                    self.tokens.push(Box::new(TokenOperator::new(
                        TokenType::BinaryOperator,
                        "*".to_string()
                    )));
                }
                TokenType::Equal => token_type = TokenType::Name,
                _ => {}
            }
        }

        self.tokens.push(Box::new(Token::new(
            token_type,
            name
        )));
    }

    fn make_token_operator(&mut self) {
        let last_token = self.tokens.last();
        let is_unary = match last_token {
            Some(token) => {
                match token.get_token_type() {
                    TokenType::BinaryOperator => true,
                    TokenType::UnaryOperator => true,
                    TokenType::EndOfExpression => true,
                    TokenType::LeftParenthesis => true,
                    TokenType::Comma => true,
                    TokenType::Name => true,
                    _ => last_token.is_none(),
                }
            }
            None => true,
        };
    
        if is_unary {
            self.tokens.push(Box::new(TokenOperator::new(
                TokenType::UnaryOperator,
                self.current_char.to_string(),
            )));
        } else {
            self.tokens.push(Box::new(TokenOperator::new(
                TokenType::BinaryOperator,
                self.current_char.to_string(),
            )));
        }
    }
}
