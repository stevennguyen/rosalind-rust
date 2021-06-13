use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filecontent = fs::read_to_string(filename)
        .expect("Could not read file.");

    let mut inputs = filecontent.lines().next().unwrap().split(" ");

    let n = inputs.next().unwrap().to_string().parse::<u64>().unwrap();
    let k = inputs.next().unwrap().to_string().parse::<u64>().unwrap();

    let mut pop_juvenile: u64 = 1;
    let mut pop_mature: u64 = 0;
    let mut pop_born: u64;

    for _ in 1..n {
        pop_born = pop_mature * k;                 // only mature pairs are able to breed, which produce a litter of k pairs
        pop_mature = pop_mature + pop_juvenile;    // the new mature population is the current mature population + previous juvenile population
        pop_juvenile = pop_born;                   // the new juvenile population is the current born population
    }

    println!("{}", pop_juvenile + pop_mature);     // the total population of juvenile + mature pairs
}