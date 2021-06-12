use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<u64>().unwrap();
    let k = args[2].parse::<u64>().unwrap();
    let mut pop_juvenile = 1;
    let mut pop_mature = 0;
    let mut pop_born: u64;

    for _ in 1..n {
        pop_born = pop_mature * k;                 // only mature pairs are able to breed, which produce a litter of k pairs
        pop_mature = pop_mature + pop_juvenile;    // the new mature population is the current mature population + previous juvenile population
        pop_juvenile = pop_born;                   // the new juvenile population is the current born population
    }

    println!("{}", pop_juvenile + pop_mature);     // the total population of juvenile + mature pairs
}