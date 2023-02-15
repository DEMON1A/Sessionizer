use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Error;
use serde_json::{from_str, Value};

pub fn read_json_session(session_string: &str) -> Result<Value, serde_json::Error> {
    let value: Value = from_str(session_string).unwrap();
    Ok(value)
}

pub fn read_file_lines(filename: &str) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}