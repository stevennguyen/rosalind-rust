use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filecontent = fs::read_to_string(filename)
        .expect("Could not read file.");

    let rev: String = filecontent.chars().rev().collect();
    let mut revc = String::new();

    for c in rev.chars() {
        match c {
            'A' => revc.push_str("T"),
            'T' => revc.push_str("A"),
            'C' => revc.push_str("G"),
            'G' => revc.push_str("C"),
            _ => (),
        }
    }

    println!("{}", revc);
}