use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect();

    // if no filename and word are provided, print usage and exit
    if args.len() < 3 {
        eprintln!("Usage: {} <filename> <word>", args[0]);
        return;
    }

    // get filename and word from command line
    let filename = &args[1];
    let word = &args[2];

    // open file
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            return;
        }
    };
    let reader = BufReader::new(file);

    // count word occurrences
    let mut count = 0;
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading file {}: {}", filename, e);
                return;
            }
        };
        let words = line.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation());
        for w in words {
            if w.to_lowercase() == *word {
                count += 1;
            }
        }
    }

    // print result
    println!("The word {} appears {} times in file {}.", word, count, filename);
}
