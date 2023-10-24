use crate::errors::{Error, ErrorType};

pub struct Math;

impl Math {
    pub(crate) fn sqrt(args: Vec<i64>) -> i64 {
        if args.len() != 1 {
            Error::throw::<()>(ErrorType::InvalidOperation);
        }
        let arg = args[0] as f64;
        arg.sqrt() as i64
    }

    pub(crate) fn max(args: Vec<i64>) -> i64 {
        if args.len() != 2 {
            Error::throw::<()>(ErrorType::InvalidOperation);
        }
        args[0].max(args[1])
    }

    pub(crate) fn min(args: Vec<i64>) -> i64 {
        if args.len() != 2 {
            Error::throw::<()>(ErrorType::InvalidOperation);
        }
        args[0].min(args[1])
    }

    pub(crate) fn facto(args: Vec<i64>) -> i64 {
        if args.len() != 1 || args[0] < 0 {
            Error::throw::<()>(ErrorType::InvalidOperation);
        }
        let mut result = 1;
        for i in 1..(args[0] as i64 + 1) {
            result *= i as i64;
        }
        result
    }

    //make isprime function and return 1 if args[0] is prime or 0 if not
    pub(crate) fn prime(args: Vec<i64>) -> i64 {
        if args.len() != 1 || args[0] < 0 {
            Error::throw::<()>(ErrorType::InvalidOperation);
        }
        let mut result = 1;
        for i in 2..(args[0] as i64) {
            if args[0] as i64 % i == 0 {
                result = 0;
                break;
            }
        }
        result
    }

    pub(crate) fn fibo(args: Vec<i64>) -> i64 {
        if args.len() != 1 || args[0] < 0 {
            Error::throw::<()>(ErrorType::InvalidOperation);
        }
        let mut result = 1;
        let mut prev = 0;
        for _ in 1..args[0] as i64 {
            let tmp = result;
            result += prev;
            prev = tmp;
        }
        result
    }

    pub(crate) fn gcd(args: Vec<i64>) -> i64 {
        if args.len() != 2 {
            Error::throw::<()>(ErrorType::InvalidOperation);
        }
        let mut x = args[0] as i64;
        let mut y = args[1] as i64;

        while y != 0 {
            let r = x % y;
            x = y;
            y = r;
        }
        x.abs() as i64
    }
}