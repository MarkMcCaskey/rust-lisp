#[macro_use]
extern crate nom;

mod types;
pub mod ast;
pub mod parser;

fn main() {
    use std::io::{stdin,stdout,Write};
    let mut s = String::new();
    loop {
        s.clear();
        stdin().read_line(&mut s).expect("Could not read from stdin");
        let output = parser::form(&s.as_bytes());
        match output {
            nom::IResult::Done(_, p) => println!("SUCCESS:\n{:?}",p),
            nom::IResult::Error(e) => println!("Error:\n{}", e),
            e => println!("Incomplete:\n{:?}", e),
        }
        stdout().flush();
    }
}
