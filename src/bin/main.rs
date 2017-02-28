#[macro_use]
extern crate daspel_rs;

use daspel_rs::parser;

fn main() {
    let res = parser::expr(b"(1.14 * -1) + 2 / false -true * nil");
    println!("{:?}", res);
    println!("{:?}", res.map(|x| format!("{}", x)));
}