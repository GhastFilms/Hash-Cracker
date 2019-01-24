
mod alphabet;
#[allow(unused_imports)]
use std::{env, str, fs::File, io::{BufRead, BufReader, prelude::*}, collections::HashSet};
//use sha2::{Sha256, Digest};
use crate::alphabet::alphabet::*;
use hashcracker;

fn main() {
    hashcracker::run();
}