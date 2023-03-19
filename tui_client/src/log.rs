#![allow(unused)]

use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn write(content: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/tmp/foo.txt")
        .unwrap();

    writeln!(file, "{}", content).unwrap();
}
