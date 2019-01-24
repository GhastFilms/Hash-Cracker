mod alphabet;
#[allow(unused_imports)]
use std::{env, str, fs::File, io::{BufRead, BufReader, prelude::*}, collections::HashSet};
use crate::alphabet::alphabet::*;
mod hasher;
use crate::hasher::*;


pub fn check(mut h: &mut hasher::Hasher, prefix: &String) {
    println!("{}", prefix);
    match h.check(&prefix) {
        Some(s) => {
            match h.output.write(s.as_bytes()) {
                Ok(_) => {},
                Err(e) => println!("could not write \"{}\" into {} \n error: {}", s, "results.txt", e),
            };
            if h.hashes.is_empty() {
                println!("found all hashes, check {}\nexiting...", "results.txt");
                return
            }
        },
        None => {},
    };
}


pub fn print_all_k_length_rec(mut h: &mut hasher::Hasher, set: Vec<&'static str>, prefix: String, k: i64) {
    if k == 0 {
        check(&mut h, &prefix);
        return
    } else if k == 1 {
        const THREAD_COUNT: i32 = 4;
        let x: i32 = (set.len() as i32 / THREAD_COUNT) as i32;
        for y in 0..x {

        }
    } 
    for x in 0..set.len() {
        print_all_k_length_rec(&mut h, set.clone(), 
            format!("{}{}", prefix, set[x as usize]), k-1
        );
    }
}


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


pub fn run() {
    const COUNT: i64 = 5;

    let output_file: File = match File::create("results.txt") {
        Ok(s) => s,
        Err(e) => {
            panic!("could not create or overide {}\n error: {}", "results.txt", e);
        },
    };
    let mut h: Hasher = Hasher::new(output_file);
    h.hashes = match get_hashes(String::from("hashes.txt")) {
        Ok(s) => s,
        Err(e) => panic!(e),
    };
            
    println!("{:?}", h.hashes);
    let characters: Vec<&'static str> = get_ascii();
    // this is all of the ascii characters
    print_all_k_length_rec(&mut h, characters.clone(), String::from(""), COUNT);
}