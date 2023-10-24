use std::any::Any;
use crate::errors::{Error, ErrorType};
use crate::lexer::tokens::{TokenTrait, TokenType};
use crate::maths::Math;

#[derive(Debug, Clone, Copy)]
pub enum DefinedFunction {
    Sqrt,
    Max,
    Min,
    Facto,
    Prime,
    Fibo,
    Gcd,
}

impl DefinedFunction {

    //get corresponding function args count
    pub fn get_args_count(&self) -> usize {
        match self {
            DefinedFunction::Sqrt => 1,
            DefinedFunction::Max => 2,
            DefinedFunction::Min => 2,
            DefinedFunction::Facto => 1,
            DefinedFunction::Prime => 1,
            DefinedFunction::Fibo => 1,
            DefinedFunction::Gcd => 2,
        }
    }

    //return true if name is a defined function
    pub fn is_defined_function(name: &str) -> Option<DefinedFunction> {
        match name {
            "sqrt" => Some(DefinedFunction::Sqrt),
            "max" => Some(DefinedFunction::Max),
            "min" => Some(DefinedFunction::Min),
            "facto" => Some(DefinedFunction::Facto),
            "isprime" => Some(DefinedFunction::Prime),
            "fibo" => Some(DefinedFunction::Fibo),
            "gcd" => Some(DefinedFunction::Gcd),
            _ => None
        }
    }
}

pub struct TokenFunction {
    pub(crate) token_type: TokenType,
    pub(crate) value: String,
    pub(crate) defined_function: DefinedFunction,
    args_count: usize,
}

impl TokenFunction {
    pub(crate) fn new(token_type: TokenType, value: String) -> Self {
        let defined_function = DefinedFunction::is_defined_function(&value);
        let args_count = match defined_function {
            Some(defined_function) => defined_function.get_args_count(),
            None => Error::throw(ErrorType::SyntaxError)
        };

        match defined_function {
            Some(defined_function) => Self {token_type, value, defined_function, args_count},
            None => Error::throw(ErrorType::SyntaxError)
        }
    }

    pub fn execute(&self, args: Vec<i64>) -> i64 {
        match self.defined_function {
            DefinedFunction::Sqrt => Math::sqrt(args),
            DefinedFunction::Max => Math::max(args),
            DefinedFunction::Min => Math::min(args),
            DefinedFunction::Facto => Math::facto(args),
            DefinedFunction::Prime => Math::prime(args),
            DefinedFunction::Fibo => Math::fibo(args),
            DefinedFunction::Gcd => Math::gcd(args),
        }
    }

    pub fn get_args_count(&self) -> usize {
        self.args_count
    }
}

impl TokenTrait for TokenFunction {
    fn get_token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

    fn set_token_type(&mut self, token_type: TokenType) {
        self.token_type = token_type;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}