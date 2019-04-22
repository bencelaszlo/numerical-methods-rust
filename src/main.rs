//#![feature(duration_float)]

mod integration;
mod interpolation;
mod sorting;
mod utils;

use std::time::{SystemTime, Duration};

fn main() {
	let start_time = SystemTime::now();
    let mut test_numbers: Vec<f32> = Vec::new();
    let _file_read_result =
        utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);
    println!("SORTING");
    for k in 0..test_numbers.len() {
        print!("{}, ", test_numbers[k]);
    }

    let mut test_numbers: Vec<f32> = Vec::new();
    let mut test_numbers: Vec<f32> = Vec::with_capacity(40000000);
    let _file_read_result =
        utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);
   	println!("\n\nShell Sort");
    sorting::shell_sort(test_numbers.len() as i32, &mut test_numbers);
   	 println!("{}", test_numbers.len());
    for k in 0..test_numbers.len() {
        print!("{}, ", test_numbers[k]);
   }

    let mut test_numbers: Vec<f32> = Vec::new();
    let _file_read_result =
        utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);
    println!("\n\nHeap Sort");
    sorting::heapsort(test_numbers.len(), &mut test_numbers);
    for k in 0..test_numbers.len() {
        print!("{}, ", test_numbers[k]);
    }

    let mut test_numbers: Vec<f32> = Vec::new();
    let _file_read_result =
        utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);
    println!("\n\nQuicksort");
    sorting::quicksort(0, (test_numbers.len() - 1) as i32, &mut test_numbers);
    for k in 0..test_numbers.len() {
        print!("{}, ", test_numbers[k]);
    }

    println!("\n\n\n");

    println!("INTERPOLATION");

    let mut x: Vec<f32> = Vec::new();
    let _file_read_result = utils::read_numbers_from_file("input.txt".to_string(), &mut x);
    println!("\n\nLinear Interpolation");
    let x_known: [f32; 7] = [-250.0, -200.0, -100.0, 0.0, 100.0, 200.0, 250.0];
    let y_known: [f32; 7] = [0.0, 280.0, 540.0, 640.0, 1200.0, 1600.0, 1800.0];
   	let mut y: Vec<f32> = Vec::new();
    interpolation::linear_interpolation_vec(&x_known, &y_known, test_numbers, &mut y);
    for k in 0..y.len() {
        print!("{}\n", y[k]);
    }

    println!("\n\n\n");

    println!("INTEGRATION OF FUNCTIONS");
    let j_max = utils::read_number_from_file("input_integration.txt".to_string()).unwrap();

	let start_time = SystemTime::now();
	
    println!("\n\nTrapedozial Rule");
    println!("{}", integration::trapedozial_rule(0.0, 3.0, 40));

    println!("\n\nIterative Trapedozial Rule");
    let integration_result = integration::q_trapedozial_rule(0.0, 40.0, j_max);
    println!("{}", integration_result);
	
    println!("\n\nSimpson's Rule");
    println!("{}", integration::q_simpsons_rule(0.0, 3.0, 40));

    println!("\n\nIterative Simpson's Rule");
    let integration_result = integration::q_simpsons_rule(0.0, 40.0, j_max);
    println!("{}", integration_result);
    println!("\n\nGauss-Legendre");
    println!("{}", integration::q_gauss_legendre(0.0, 4.0, 5));

	let end_time = SystemTime::now();
	let elapsed_time =  end_time.duration_since(start_time).expect("SystemTime::duration_since failed");
	println!("{:?}", elapsed_time.as_millis() as f64 / 1000.0);
}
