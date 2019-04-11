use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn read_numbers_from_file<T>(
    file_path: String,
    numbers_vector: &mut Vec<T>,
) -> std::io::Result<()>
where
    T: FromStr,
    T: Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        let number: T = line.parse().unwrap();
        numbers_vector.push(number);
    }

    Ok(())
}

pub fn read_number_from_file(file_path: String) -> std::io::Result<i32> {
    let mut number: i32 = 0;

    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        let last_number: i32 = line.parse::<i32>().unwrap();

        number = last_number;
    }

    Ok(number)
}
