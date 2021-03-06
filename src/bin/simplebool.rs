extern crate tapl;

use std::io;
use std::io::Write;

use tapl::tapl::simplebool;

fn main() {
    let mut input = String::new();
    let context = simplebool::Context::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("read_line error");
        input.trim();
        match simplebool::repl(&input, &context) {
            Ok(s) => println!("{}", s),
            Err(e) => println!("{:?}", e),
        }
        input.clear();
    }
}
