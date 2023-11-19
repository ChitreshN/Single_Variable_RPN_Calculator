use core::panic;

enum Operators {
    Expo,
    Div,
    Mul,
    Add,
    Sub,
    Diff,
}

fn eval(x: f32, y: f32, op: Operators) -> Option<f32> {
    let x = match op {
        Operators::Expo => x.powf(y.try_into().unwrap()),
        Operators::Mul => x * y,
        Operators::Div =>{
            if y == 0.0{
                println!("NAN");
                return None
            }
            x / y
        } ,
        Operators::Add => x + y,
        Operators::Sub => x - y,
        Operators::Diff => x,
    };
    Some(x)
}

fn oper(x : &str)-> Operators{
    match x{
        "^" => Operators::Expo,
        "/" => Operators::Div,
        "*" => Operators::Mul,
        "-" => Operators::Sub,
        "+" => Operators::Add,
        "d" => Operators::Diff,
         _  => panic!("Invalid operator"),
    }
}

pub fn exp_eval(expression: String) -> Option<f32>{
    let mut stack = Vec::new();
    let parts = expression.split_whitespace();
    for part in parts {
        match part {
                  "^"
                | "*"
                | "/"
                | "-"
                | "+"=> {
                    let x = stack.pop();
                    let x1 : f32;
                    match x {
                        Some(y) => x1 = y,
                        None =>{
                            println!("Invalid Expression");
                            return None
                        }
                    }
                    let y = stack.pop();
                    let y1 : f32;
                    match y {
                        Some(y) => y1 = y,
                        None =>{
                            println!("Invalid Expression");
                            return None
                        }
                    }
                    let res = eval(y1,x1,oper(part));
                    match res {
                        Some(x) => stack.push(x),
                        None => return None
                    }
                }
            _ => {
                if part.chars().nth(0) == Some('-'){
                    let num: f32 = part[1..].trim().parse().expect("Not valid number");
                    stack.push(-(num));
                }
                else {
                    let num: f32 = part.trim().parse().expect("Not valid number");
                    stack.push(num);
                }
            }
        }
    }
    let res = stack.pop().expect("Invalid expression");
    if stack.len() != 0 {
        println!("Invalid Expression");
        return None
    }
    Some(res)
}
