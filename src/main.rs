use std::env::args;
use std::io::{self, Write};

fn main() {
    let arg: Vec<_> = args().collect();
    if arg.len() <= 1{
        println!("Usage: echo \"msg\"");
    }

    let mut index = 1;
    let arg_length = arg.len();
    while index<arg_length {
        io::stdout().flush().unwrap();
        print!("{} ",arg[index]);
        index += 1;
    }
}
