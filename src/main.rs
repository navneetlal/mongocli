use std::env;
// use std::fs;
// use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    // if args[1] == "configure" {
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input)?;
    //     println!("You typed: {}", input.trim());
    // }
    println!("{:?}", args);
}
