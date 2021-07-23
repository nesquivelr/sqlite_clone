use std::io::{self, Write};
use std::process;

fn print_prompt(){
    print!("db >");
    io::stdout()
        .flush()
        .unwrap();
}


fn main(){
    loop {
        let mut input = String::new();
        print_prompt();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        if input.eq(".exit"){
            process::exit(0);
        } else {
            println!("Unrecognized command '{}'", input);
        }
    }
}
