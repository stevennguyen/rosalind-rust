use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filecontent = fs::read_to_string(filename)
        .expect("Could not read file.");

    let mut inputs = filecontent.lines();

    let s = inputs.next().unwrap();
    let t = inputs.next().unwrap();

    let mut s_char = s.chars();
    let mut t_char = t.chars();

    let mut hamming = 0;

    for _ in s.chars() {
        if s_char.next().unwrap() != t_char.next().unwrap() {
            hamming += 1;
        }
    }

    println!("{}", hamming);
}