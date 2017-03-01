#[macro_use]
extern crate daspel_rs;

use daspel_rs::parser;

fn main() {
    let res = parser::run(b"(1.14 * -1) + 2 / # haha cucks    \n\nfalse -true & nil");
    println!("{:?}", res);
    println!("{:?}", res.map(|x| format!("{}", x)));
}