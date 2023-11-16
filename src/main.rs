use std::{io, process::exit};
pub mod calc;

fn main() {
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read");
    let res = calc::exp_eval(expression);
    println!("{}", res);
    exit(0);
}
