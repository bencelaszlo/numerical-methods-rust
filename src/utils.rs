use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_numbers_from_file(file_path: String, numbers_vector: &mut Vec<f32>) -> std::io::Result<()> {
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        let number: f32 = line.parse().unwrap();
        numbers_vector.push(number);
    }

    Ok(())
}