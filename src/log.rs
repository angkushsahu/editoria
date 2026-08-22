// todo: remove this file before sending to production
use std::{fs::OpenOptions, io::Write};

#[allow(dead_code)]
pub fn log(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")
        .unwrap();

    writeln!(file, "{message}").unwrap();
}
