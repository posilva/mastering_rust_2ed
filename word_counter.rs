use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::collections::HashMap;


#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);


impl WordCounter {
    fn new() -> WordCounter {
        return WordCounter(HashMap::new());
    } 

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(&self, filter:u64) {
        let mut v: Vec<_> = self.0.iter().collect();
        v.sort_by(|x,y| x.1.cmp(&y.1));
        println!("{:?}", v);
        for (key, value) in v {
            if value > &filter {
                println!("{}: {}", key, value);
            }
        }
    }

}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = &arguments[1];
    println!("Processing file: {}", filename);

    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue
            } else {
                word_counter.increment(word);
            }
        }
    }
    word_counter.display(0);
}

