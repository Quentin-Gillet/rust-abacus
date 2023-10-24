use crate::errors::{Error, ErrorType};
use crate::lexer::tokens::{TokenTrait, TokenType};
use crate::lexer::tokens::binary_operator_token::{Associativity, TokenOperator};

pub(crate) struct ShuntingYard {
    pub output_queue: Vec<Box<dyn TokenTrait>>,
    operator_stack: Vec<Box<dyn TokenTrait>>,
}

impl ShuntingYard {
    pub fn new() -> Self {
        Self {
            output_queue: Vec::new(),
            operator_stack: Vec::new()
        }
    }

    pub fn process(&mut self, tokens: Vec<Box<dyn TokenTrait>>){
        for token in tokens {
            match token.get_token_type() {
                TokenType::Number | TokenType::Variable => {
                    self.output_queue.push(token);
                    if !self.operator_stack.is_empty() && self.operator_stack.last().unwrap().get_token_type() == TokenType::UnaryOperator {
                        self.output_queue.push(self.operator_stack.pop().unwrap());
                    }
                }
                TokenType::UnaryOperator => {
                    self.operator_stack.push(token);
                }
                TokenType::Function | TokenType::Name => {
                    self.operator_stack.push(token);
                }
                TokenType::Comma => {
                    if !self.operator_stack.is_empty() &&
                        self.operator_stack.last().unwrap().get_token_type() == TokenType::Function {
                        self.output_queue.push(self.operator_stack.pop().unwrap());
                    }
                }
                TokenType::BinaryOperator => {
                    while !self.operator_stack.is_empty() &&
                        matches!(self.operator_stack.last().unwrap().get_token_type(),
                         TokenType::BinaryOperator | TokenType::UnaryOperator) {

                        let top = self.operator_stack.last().unwrap();

                        let o1 = token.as_any().downcast_ref::<TokenOperator>()
                            .expect("Failed to downcast TokenOperator");
                        let o2 = top.as_any().downcast_ref::<TokenOperator>()
                            .expect("Failed to downcast TokenOperator");

                        if o2.get_precedence() > o1.get_precedence() ||
                            (o2.get_precedence() == o1.get_precedence() && o1.get_associativity() == Associativity::Left) {
                            self.output_queue.push(self.operator_stack.pop().unwrap());
                            continue;
                        }
                        break;
                    }
                    self.operator_stack.push(token);
                }
                TokenType::LeftParenthesis => {
                    self.operator_stack.push(token);
                }
                TokenType::RightParenthesis => {
                    let mut found: bool = false;

                    while !self.operator_stack.is_empty() {
                        let top = self.operator_stack.last().unwrap();
                        if top.get_token_type() == TokenType::LeftParenthesis {
                            self.operator_stack.pop();

                            if !self.operator_stack.is_empty() {
                                let top = self.operator_stack.last().unwrap();
                                if top.get_token_type() == TokenType::Function {
                                    self.output_queue.push(self.operator_stack.pop().unwrap());
                                }
                            }
                            found = true;
                            break;
                        }
                        self.output_queue.push(self.operator_stack.pop().unwrap());
                    }

                    if !found && self.operator_stack.is_empty() {
                        Error::throw::<()>(ErrorType::SyntaxError);
                    }
                }
                TokenType::EndOfExpression => {
                    while !self.operator_stack.is_empty() {
                        let top = self.operator_stack.last().unwrap();
                        if top.get_token_type() == TokenType::LeftParenthesis {
                            Error::throw::<()>(ErrorType::SyntaxError);
                        }
                        self.output_queue.push(self.operator_stack.pop().unwrap());
                    }
                    self.output_queue.push(token);
                }
                TokenType::Equal => self.operator_stack.push(token),
            }
        }
        while !self.operator_stack.is_empty() {
            let top = self.operator_stack.last().unwrap();
            if top.get_token_type() == TokenType::LeftParenthesis {
                Error::throw::<()>(ErrorType::SyntaxError);
            }
            self.output_queue.push(self.operator_stack.pop().unwrap());
        }
    }


}