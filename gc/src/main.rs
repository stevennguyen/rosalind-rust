use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filecontent = fs::read_to_string(filename)
        .expect("Could not read file.");

    let mut gc_dna_name = String::new();
    let mut gc_gc: f64 = 0.0;

    for dna_entry in filecontent.split(">") {
        let mut dna_name = String::new();
        let mut dna = String::new();
        let gc: f64;

        for line in dna_entry.lines() {
            if line.contains("Rosalind") {
                dna_name = line.to_string();
            } else {
                dna = dna + line;
            }
        }

        gc = (dna.matches('G').count() + dna.matches('C').count()) as f64 * 100.0 / dna.chars().count() as f64;
        if gc > gc_gc {
            gc_dna_name = dna_name;
            gc_gc = gc;
        }
    }

    println!("{}\n{:.6}", gc_dna_name, gc_gc);
}