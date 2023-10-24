use std::collections::VecDeque;
use crate::errors::{Error, ErrorType};
use crate::lexer::tokens::{TokenTrait, TokenType};
use crate::lexer::tokens::binary_operator_token::TokenOperator;
use crate::lexer::tokens::function_token::TokenFunction;

pub struct Executor {
    number_stack: Vec<i64>,
    pub vars: Vec<(String, i64)>,
    pub token_history: Vec<i64>
}

impl Executor {
    pub fn new() -> Self {
        Self {number_stack: Vec::new(), vars: Vec::new(), token_history: Vec::new()}
    }

    pub fn execute(&mut self, tokens: Vec<Box<dyn TokenTrait>>) -> i64{
        for (i, token) in tokens.iter().enumerate() {
            let mut result: i64 = 0;
            if token.get_token_type() == TokenType::EndOfExpression {
                if i == tokens.len() - 1 {
                    return *self.number_stack.first().unwrap();
                }
                else {
                    self.number_stack.clear();
                    continue;
                }
            }
            else if token.get_token_type() == TokenType::Number {
                result = token.get_value().parse::<i64>().unwrap();
            }
            else {
                match token.get_token_type() {
                    TokenType::BinaryOperator => {
                        let operator = token.as_any().downcast_ref::<TokenOperator>()
                            .expect("Failed to downcast TokenOperator");

                        if self.number_stack.len() < 2 {
                            Error::throw::<()>(ErrorType::SyntaxError);
                        }
                        let number1 = self.number_stack.pop().unwrap();
                        let number2 = self.number_stack.pop().unwrap();

                        result = operator.execute(number2, number1);
                    }
                    TokenType::UnaryOperator => {
                        let number1 = self.number_stack.pop().unwrap();
                        let operator = token.as_any().downcast_ref::<TokenOperator>()
                            .expect("Failed to downcast TokenOperator");
                        result = operator.execute(number1, 0);
                    }
                    TokenType::Function => {
                        let operator = token.as_any().downcast_ref::<TokenFunction>()
                            .expect("Failed to downcast TokenFunction");
                        let mut args: VecDeque<i64> = VecDeque::new();
                        for _ in 0..operator.get_args_count(){
                            let number = self.number_stack.pop().unwrap();
                            args.push_front(number)
                        }
                        let args = args.into_iter().collect::<Vec<i64>>();
                        result = operator.execute(args);
                    }
                    TokenType::Name => {
                        if let Some(operator) = tokens.get(i + 1) {
                            if operator.get_token_type() == TokenType::BinaryOperator {
                                Error::throw::<()>(ErrorType::SyntaxError);
                            }
                        }
                        if let Some(value) = self.number_stack.last() {
                            if let Some(index) = self.vars.iter().position(|(name, _)| name == &token.get_value()) {
                                self.vars.get_mut(index).unwrap().1 = *value;
                            }
                            else { self.vars.push((token.get_value(), *value)); }
                        }
                        continue;
                    }
                    TokenType::Variable => {
                        if self.vars.iter().any(|(name, _)| name == &token.get_value()) {
                            result = self.vars.
                                iter().
                                find(|(name, _)| name == &token.get_value()).
                                unwrap().1;
                        }
                        else { Error::throw::<()>(ErrorType::UnboundVariable); }
                    }
                    _ => {}
                }
            }

            self.token_history.push(result);
            self.number_stack.push(result);
        }
        if self.number_stack.len() > 1 { Error::throw::<()>(ErrorType::SyntaxError); }

        match self.number_stack.pop() {
            Some(number) => number,
            None => 0
        }
    }
}
