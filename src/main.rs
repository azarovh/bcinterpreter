extern crate bcinterpreter;

use std::{env, fs, process};

use bcinterpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Wrong number of arguments. Expected: [path]; actual:{:?}",
            args
        );
        process::exit(1);
    }

    let file = &args[1];

    if let Ok(s) = fs::read_to_string(&file) {
        println!("Running bytecode interpreter...");
        let mut bc = Interpreter::new(&s);
        println!("The result is {}", bc.run().unwrap());
    } else {
        println!("Couldn't read file {}", &file);
    }
}
