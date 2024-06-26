#![allow(unused)]

use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn write(content: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("/tmp/mahjong_log.txt")
        .unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .open("/tmp/mahjong_log.txt")
        .unwrap();

    writeln!(file, "{}", content).unwrap();
}
