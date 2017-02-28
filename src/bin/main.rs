#[macro_use]
extern crate daspel_rs;

use daspel_rs::parser;

fn main() {
    let res = parser::expr(b"(1 * -1) + 2");
    println!("{:?}", res);
    println!("{:?}", res.map(|x| format!("{}", x)));
}