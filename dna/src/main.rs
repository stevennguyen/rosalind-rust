use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filecontent = fs::read_to_string(filename)
        .expect("Could not read file.");

    println!("{} {} {} {}", filecontent.matches('A').count(), filecontent.matches('C').count() , filecontent.matches('G').count() ,filecontent.matches('T').count());
}