pub mod tests;
use std::{env, str, fs::File, collections::HashSet};
use sha2::{Sha256, Digest};
   
pub struct Hasher {
    pub output: File,
    pub hashes: HashSet<String>,
    //finished: bool,
}   

impl Hasher {
    ///converts an input vector of bytes to a hex string
    /// # Example
    /// ```
    /// let x: String = String::from("hello world");
    /// let hex_string: String = String::from("68656C6C6F20776F726C64");
    /// 
    /// assert_eq!(hex_string, hashcracker::hasher::Hasher::to_hex(x.as_bytes().to_vec()));
    /// ```
    pub fn to_hex(input: Vec<u8>) -> String {
        let result: Vec<String> = input.iter().map(|b| format!("{:02X}", b)).collect();
        result.concat() as String
    }

    ///takes a String and returns the sha256 hash of the string
    /// 
    /// # Example
    /// ```
    /// let x: String = String::from("hello world");
    /// let hash: String = String::from("B94D27B9934D3E08A52E52D7DA7DABFAC484EFE37A5380EE9088F7ACE2EFCDE9");
    /// 
    /// assert_eq!(hash, hashcracker::hasher::Hasher::hash(x));
    /// ```
    pub fn hash(input: String) -> String {
        let mut hasher = Sha256::new();
        hasher.input(input);
        Hasher::to_hex(hasher.result().to_vec())
    }

    ///checks if a strings hash is in the hashes HashSet
    ///returns an Option<String> containing Some(String) if the hash was in the list and None if not
    pub fn check(&mut self, i: &String) -> Option<String> {
        let result: String = Hasher::hash(i.clone());
        if self.hashes.contains(&result) {
            self.hashes.remove(&result);
            Some(format!("{}: {}\n", result, i))
        } else {
            Option::None
        }
    }
    
    ///New creates a new Hasher struct
    ///takes in a std::fs::File that will be used for writing the output
    pub fn new(outfile: File) -> Hasher {
        Hasher {
            hashes: HashSet::new(),
            output: outfile,
        }
    }
}