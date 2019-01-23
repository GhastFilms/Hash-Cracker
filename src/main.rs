mod alphabet;

use std::{env, str, fs::File, process, io::{BufRead, BufReader, prelude::*}, collections::HashSet};
use sha2::{Sha256, Digest};
use crate::alphabet::alphabet::*;

const COUNT: i64 = 5;

fn get_hashes(name: String) -> Result<HashSet<String>, String> {
    let file = match File::open(name) {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };
    let file = BufReader::new(file);
    let mut r: HashSet<String> = HashSet::new();
    for line in file.lines() {
        r.insert(line.unwrap());
    };
    Ok(r)
}

fn main() {
    let mut h: Hasher = Hasher::new();
    h.hashes = match get_hashes(String::from("hashes.txt")) {
        Ok(s) => s,
        Err(e) => panic!(e),
    };
    println!("{:?}", h.hashes);
    let characters: Vec<&'static str> = get_lowercase();
    // this is all of the ascii characters
    print_all_k_length_rec(&mut h, characters.clone(), String::from(""), characters.len() as i64, COUNT);
}


fn print_all_k_length_rec(mut h: &mut Hasher, set: Vec<&'static str>, prefix: String, n: i64, k: i64) {
    if k == 0 {
        h.check(&prefix);
        return
    }
    for x in 0..n {
        print_all_k_length_rec(&mut h, set.clone(), 
                            format!("{}{}", prefix, set[x as usize]), 
                            n, k-1
                          );
    }
}

struct Hasher {
    file: File,
    hashes: HashSet<String>,
}

impl Hasher {
    pub fn check(&mut self, i: &String) {
        let mut hasher = Sha256::new();
        hasher.input(i);
        let result: Vec<String> = hasher.result().iter().map(|b| format!("{:02X}", b)).collect();
        let result: String = result.concat();
        
        if self.hashes.contains(&result) {
            self.hashes.remove(&result);
            let result = format!("{}: {}\n", result, i);
            
            match self.file.write(result.as_bytes()) {
                Ok(_) => {},
                Err(e) => println!("could not write \"{}\" into {} \n error: {}", result, "results.txt", e),
            };

            if self.hashes.is_empty() {
                println!("found all hashes, check {}\nexiting...", "results.txt");
                process::exit(1);
            }
        };
    }

    pub fn new() -> Hasher {
        Hasher {
            hashes: HashSet::new(),
            file: {
                match File::create("results.txt") {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("{}", e);
                        process::exit(1);
                    },
                }
            },
        }
    }
}

