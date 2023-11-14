use std::io;

// expression???
// const
// var
// exp * exp * PLUS
// exp * exp * MUL
// exp * exp * DIV
// exp * exp * SUB

enum Operators {
    Expo,
    Div,
    Mul,
    Add,
    Sub,
}

struct Exp {
    exp1: Box<Exp>,
    exp2: Box<Exp>,
    oper: Operators,
}

fn eval(x: u32, y: u32, op: Operators) -> u32 {
    match op {
        Operators::Expo => x.pow(y),
        Operators::Mul => x * y,
        Operators::Div => x / y,
        Operators::Add => x + y,
        Operators::Sub => x - y,
    }
}

fn exp_eval(expression: String) -> u32 {
    let mut stack = Vec::new();
    // iterator
    let parts = expression.split_whitespace();
    for part in parts {
        match part {
            "^" => {
                let x: u32 = stack.pop().expect("Stack empty");
                let y: u32 = stack.pop().expect("Stack empty");
                stack.push(eval(y, x, Operators::Expo));
            }
            "*" => {
                let x: u32 = stack.pop().expect("Stack empty");
                let y: u32 = stack.pop().expect("Stack empty");
                stack.push(eval(y, x, Operators::Mul));
            }
            "/" => {
                let x: u32 = stack.pop().expect("Stack empty");
                let y: u32 = stack.pop().expect("Stack empty");
                stack.push(eval(y, x, Operators::Div));
            }
            "-" => {
                let x: u32 = stack.pop().expect("Stack empty");
                let y: u32 = stack.pop().expect("Stack empty");
                stack.push(eval(y, x, Operators::Sub));
            }
            "+" => {
                let x: u32 = stack.pop().expect("Stack empty");
                let y: u32 = stack.pop().expect("Stack empty");
                stack.push(eval(y, x, Operators::Add));
            }
            _ => {
                let num: u32 = part.trim().parse().expect("");
                stack.push(num);
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
