use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_: f32 = first.parse().unwrap();
    let second_: f32 = second.parse().unwrap();

    let res: f32 = operate(operator, first_, second_);

    println!("{}", output(first_, operator, second_, res));    
}

fn operate(operator: char, first: f32, second: f32) -> f32 {
    match operator {
        '+' => first + second,
        '-' => first - second,
        '*' | 'x' | 'X' => first * second,
        '/' => first / second,
        _ => panic!("Invalid operator: {}", operator),
    }
}

fn output(first: f32, operator: char, second: f32, result: f32) -> String {

    format!("{} {} {} = {}", first, operator, second, result)
}
