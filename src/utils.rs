use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("File doesn't exists.");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn read_numbers_from_file(file_path: String) -> Vec<f32> {
    let lines = lines_from_file(file_path);

    let mut numbers: Vec<f32> = Vec::new();

    for line in lines {
        let number: f32 = line.parse().unwrap();
        numbers.push(number);
    }

    return numbers;
}