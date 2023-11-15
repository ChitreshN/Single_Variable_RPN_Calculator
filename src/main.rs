use std::io;

// expression???
// const
// var
// exp * exp * PLUS
// exp * exp * MUL
// exp * exp * DIV
// exp * exp * SUB

// [TODO] => Modularize code

enum Operators {
    Expo,
    Div,
    Mul,
    Add,
    Sub,
}


fn eval(x: i32, y: i32, op: Operators) -> i32 {
    match op {
        Operators::Expo => x.pow(y.try_into().unwrap()),
        Operators::Mul => x * y,
        Operators::Div => x / y,
        Operators::Add => x + y,
        Operators::Sub => x - y,
    }
}

fn oper(x : &str)-> Operators{
    match x{
        "^" => Operators::Expo,
        "/" => Operators::Div,
        "*" => Operators::Mul,
        "-" => Operators::Sub,
        "+" => Operators::Add,
         _  => panic!("Invalid operator"),
    }
}

fn exp_eval(expression: String) -> i32 {
    let mut stack = Vec::new();
    // iterator
    let parts = expression.split_whitespace();
    for part in parts {
        match part {
            "^"
                | "*"
                | "/"
                | "-"
                | "+"=> {
                    let x: i32 = stack.pop().expect("Stack empty");
                    let y: i32 = stack.pop().expect("Stack empty");
                    stack.push(eval(y, x, oper(part)));
                }
            _ => {
                if part.chars().nth(0) == Some('-'){
                    let num: i32 = part[1..].trim().parse().expect("Not valid");
                    stack.push(-(num));
                }
                else {
                    let num: i32 = part.trim().parse().expect("Not valid number");
                    stack.push(num);
                }
            }
        }
    }
    let res = stack.pop().expect("Invalid expression");
    res
}

fn main() {
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read");
    let res = exp_eval(expression);
    println!("{}", res);
}
