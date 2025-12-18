use std::io::{Write, stdin, stdout};

use crate::expression::Expression;

mod expression;
fn main() {
    print!(">> ");
    stdout().flush().unwrap();
    let input = {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        buf
    };
    let res = Expression::from_str(input.as_str());
    res.print_infix();
    println!("{}", res);
    println!("{}", res.print_visual());
}
