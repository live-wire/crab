#![allow(unused)]
//  ^ This is only for tutorials

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Read};
use std::io::{BufRead, BufReader, ErrorKind, Write};

use crab::exported_module;
use crab::nest1::nest2::DummyStruct;
fn main() {
    let ds = DummyStruct::new();
    println!("{:?}", ds);
}
