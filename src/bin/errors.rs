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

    // Pattern match
    let file_open = File::open("README.md");
    match file_open {
        Ok(f) => println!("FOUND FILE {:?}", f),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("CREATE FILE {:?}", e),
            x => panic!("SOME ERROR {:?}", x),
        },
    }

    // Closure
    let file_open2 = File::open("README.md");
    let f = file_open2.unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("CREATE FILE {:?}", error);
        } else {
            panic!("SOME ERROR {:?}", error);
        }
    });
    println!("FOUND FILE {:?}", f);
    // let mut s: String = String::new();
    // io::stdin().read_line(&mut s).expect("Read error");
}

fn read_username_from_file(filepath: String) -> Result<String, io::Error> {
    let fo = File::open(filepath);
    let mut ff = match fo {
        Ok(fi) => fi,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match ff.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_clean(filepath: String) -> Result<String, io::Error> {
    let mut username: String = String::new();
    File::open(filepath)?.read_to_string(&mut username)?;
    Ok(username)
}
