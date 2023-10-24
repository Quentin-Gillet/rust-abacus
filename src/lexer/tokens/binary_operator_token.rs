use std::any::Any;
use std::fmt::{Debug};
use crate::errors::{Error, ErrorType};
use crate::lexer::tokens::{TokenTrait, TokenType};

#[derive(Debug, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum TokenOperatorValue {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulo,
}

pub struct TokenOperator {
    pub(crate) token_type: TokenType,
    pub operator_value: TokenOperatorValue,
    pub value: String,
}

impl TokenOperator {
    pub(crate) fn new(token_type: TokenType, value: String) -> Self {
        let operator_option: Option<TokenOperatorValue> = TokenOperator::get_operator_value(&value);
        match operator_option {
            Some(operator_value) => Self {token_type, value, operator_value},
            None => Error::throw(ErrorType::SyntaxError)
        }
    }

    pub fn get_operator_value(operator: &str) -> Option<TokenOperatorValue> {
        match operator {
            "+" => Some(TokenOperatorValue::Plus),
            "-" => Some(TokenOperatorValue::Minus),
            "*" => Some(TokenOperatorValue::Multiply),
            "/" => Some(TokenOperatorValue::Divide),
            "^" => Some(TokenOperatorValue::Power),
            "%" => Some(TokenOperatorValue::Modulo),
            _ => None
        }
    }

    pub fn get_precedence(&self) -> i32 {
        match self.operator_value {
            TokenOperatorValue::Plus => 1,
            TokenOperatorValue::Minus if self.token_type == TokenType::BinaryOperator => 1,
            TokenOperatorValue::Minus if self.token_type == TokenType::UnaryOperator => 4,
            TokenOperatorValue::Multiply => 2,
            TokenOperatorValue::Divide => 2,
            TokenOperatorValue::Power => 3,
            TokenOperatorValue::Modulo => 2,
            _ => Error::throw(ErrorType::SyntaxError)
        }
    }

    pub fn get_associativity(&self) -> Associativity {
        match self.operator_value {
            TokenOperatorValue::Plus => Associativity::Left,
            TokenOperatorValue::Minus if self.token_type != TokenType::UnaryOperator => Associativity::Left,
            TokenOperatorValue::Minus if self.token_type == TokenType::UnaryOperator => Associativity::Right,
            TokenOperatorValue::Multiply => Associativity::Left,
            TokenOperatorValue::Divide => Associativity::Left,
            TokenOperatorValue::Power => Associativity::Right,
            TokenOperatorValue::Modulo => Associativity::Left,
            TokenOperatorValue::Minus => Associativity::Left,
        }
    }

    pub fn execute(&self, left: i64, right: i64) -> i64 {
        match self.operator_value {
            TokenOperatorValue::Plus => left + right,
            TokenOperatorValue::Minus if self.token_type == TokenType::BinaryOperator => left - right,
            TokenOperatorValue::Minus if self.token_type == TokenType::UnaryOperator => -left,
            TokenOperatorValue::Multiply => left * right,
            TokenOperatorValue::Divide => left / right,
            TokenOperatorValue::Power => TokenOperator::power(left, right),
            TokenOperatorValue::Modulo => left % right,
            _ => Error::throw(ErrorType::SyntaxError)
        }
    }

    fn power(left: i64, right: i64) -> i64 {
        if left == 0 && right == 0 {
            return 1;
        }

        if left == 0 || right == 0 {
            Error::throw::<()>(ErrorType::InvalidOperation);
        }
        let left_f = left as f64;
        left_f.powf(right as f64) as i64
    }
}

impl TokenTrait for TokenOperator {
    fn get_token_type(&self) -> TokenType {
        self.token_type.clone()
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