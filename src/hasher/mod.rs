pub mod tests;
use std::{env, str, fs::File, collections::HashSet};
use sha2::{Sha256, Digest};
   
pub struct Hasher {
    pub output: File,
    pub hashes: HashSet<String>,
}   

impl Hasher {
    pub fn to_hex(input: Vec<u8>) -> String {
        let result: Vec<String> = input.iter().map(|b| format!("{:02X}", b)).collect();
        result.concat() as String
    }

    pub fn hash(input: &String) -> String {
        let mut hasher = Sha256::new();
        hasher.input(input);
        Hasher::to_hex(hasher.result().to_vec())
    }

    pub fn check(&mut self, i: &String) -> Option<String> {
        let result: String = Hasher::hash(i);
        if self.hashes.contains(&result) {
            self.hashes.remove(&result);
            Some(format!("{}: {}\n", result, i))
        } else {
            Option::None
        }
    }

    pub fn new(outfile: File) -> Hasher {
        Hasher {
            hashes: HashSet::new(),
            output: outfile,
        }
    }
}