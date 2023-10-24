mod lexer;
mod executor;
mod maths;
mod errors;

use std::{env, io};
use crate::executor::Executor;
use crate::lexer::Lexer;
use crate::lexer::tokens::TokenTrait;
use crate::lexer::shunting_yard::ShuntingYard;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut verbose: bool = true;
    if args.len() > 1 && args[1] == "-silent" {
        verbose = false;
    }

    // let mut user_input = "a=sqrt(150);b=gcd(845, 951)=c_;z=(c_)(b)+8;z=76z+facto(a)-(-min(b, -c))".to_string();
    let mut user_input = "7=;9+8".to_string();
    if !verbose {
        user_input = "".to_string();
        let stdin = io::stdin();
        stdin.read_line(&mut user_input).expect("TODO: panic message");
    }

    let mut lexer: Lexer = Lexer::new(&user_input);
    lexer.process();
    let tokens: Vec<Box<dyn TokenTrait>> = lexer.tokens;

    if verbose {
        println!("ORIGINAL => {:}", user_input);
        print!("TOKEN : (");
        for token in &tokens {
            token.print();
        }
        println!(")");
    }

    let mut st: ShuntingYard = ShuntingYard::new();
    st.process(tokens);
    let tokens: Vec<Box<dyn TokenTrait>> = st.output_queue;

    if verbose {
        print!("SHUNTING YARD : (");
        for token in &tokens {
            token.print();
        }
        println!(")");
    }

    let mut executor: Executor = Executor::new();
    let result = executor.execute(tokens);

    if verbose {
        let results = executor.token_history;
        print!("EXEUTOR : (");
        for result in &results {
            print!("{:?}, ", result);
        }
        println!(")");
    }

    if verbose {
        print!("VARS : (");
        for var in executor.vars.iter() {
            print!("[{:?}] => {:?}, ", var.0, var.1);
        }
        println!(")");
    }

    println!("{}", result as i64);
}
