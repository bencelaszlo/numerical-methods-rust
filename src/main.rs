mod integration;
mod interpolation;
mod sorting;
mod utils;

use std::time::{SystemTime, Duration};

fn main() {
    let mut test_numbers: Vec<f32> = Vec::new();
    let _file_read_result =
        utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);
   	println!("\n\nShell's Sort");

	let mut start_time = SystemTime::now();

    sorting::shell_sort(test_numbers.len() as i32, &mut test_numbers);

   	let mut end_time = SystemTime::now();
	let mut elapsed_time =  end_time.duration_since(start_time).expect("SystemTime::duration_since failed");

    for k in 0..test_numbers.len() {
        //print!("{}, ", test_numbers[k]);
   }
	println!("{:?}", elapsed_time.as_millis() as f64 / 1000.0);


    let mut test_numbers: Vec<f32> = Vec::new();
    let _file_read_result =
        utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);

    println!("\n\nHeap Sort");

    start_time = SystemTime::now();

    sorting::heapsort(test_numbers.len(), &mut test_numbers);

    end_time = SystemTime::now();
    elapsed_time = end_time.duration_since(start_time).expect("SystemTime::duration_since failed");

    for k in 0..test_numbers.len() {
        //print!("{}, ", test_numbers[k]);
    }

    println!("\nelapsed time: {:?}", elapsed_time.as_millis() as f64 / 1000.0);

    let mut test_numbers: Vec<f32> = Vec::new();
    let _file_read_result =
        utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);

    println!("\n\nQuicksort");

    start_time = SystemTime::now();

    sorting::quicksort(0, (test_numbers.len() - 1) as i32, &mut test_numbers);

    end_time = SystemTime::now();
    elapsed_time = end_time.duration_since(start_time).expect("SystemTime::duration_since failed");

    for k in 0..test_numbers.len() {
        //print!("{}, ", test_numbers[k]);
    }

    println!("\nelapsed time: {:?}", elapsed_time.as_millis() as f64 / 1000.0);

    println!("\n\n\n");

    println!("INTERPOLATION");

    let mut x: Vec<f32> = Vec::new();
    let _file_read_result = utils::read_numbers_from_file("input.txt".to_string(), &mut x);
    println!("\n\nLinear Interpolation");

    let x_known: [f32; 7] = [-250.0, -200.0, -100.0, 0.0, 100.0, 200.0, 250.0];
    let y_known: [f32; 7] = [0.0, 280.0, 540.0, 640.0, 1200.0, 1600.0, 1800.0];
   	let mut y: Vec<f32> = Vec::new();

    start_time = SystemTime::now();

    interpolation::linear_interpolation_vec(&x_known, &y_known, test_numbers, &mut y);

    end_time = SystemTime::now();
    elapsed_time = end_time.duration_since(start_time).expect("SystemTime::duration_since failed");

    for k in 0..y.len() {
        //print!("{}\n", y[k]);
    }

    println!("\nelapsed time: {:?}", elapsed_time.as_millis() as f64 / 1000.0);

    println!("\n\n\n");

    println!("INTEGRATION OF FUNCTIONS");
    let j_max = utils::read_number_from_file("input_integration.txt".to_string()).unwrap();

    println!("\n\nTrapedozial Rule");

    start_time = SystemTime::now();

    println!("{}", integration::trapedozial_rule(0.0, 40.0, 40));

    end_time = SystemTime::now();
    elapsed_time = end_time.duration_since(start_time).expect("SystemTime::duration_since failed");

    println!("\nelapsed time: {:?}", elapsed_time.as_millis() as f64 / 1000.0);

    println!("\n\nIterative Trapedozial Rule");

    start_time = SystemTime::now();

    let integration_result = integration::q_trapedozial_rule(0.0, 40.0, j_max);

    end_time = SystemTime::now();
    elapsed_time = end_time.duration_since(start_time).expect("SystemTime::duration_since failed");

    println!("{}", integration_result);

    println!("\nelapsed time: {:?}", elapsed_time.as_millis() as f64 / 1000.0);

    println!("\n\nIterative Simpson's Rule");

    start_time = SystemTime::now();

    let integration_result = integration::q_simpsons_rule(0.0, 40.0, j_max);

    end_time = SystemTime::now();
    elapsed_time = end_time.duration_since(start_time).expect("SystemTime::duration_since failed");

    println!("{}", integration_result);

    println!("\nelapsed time: {:?}", elapsed_time.as_millis() as f64 / 1000.0);

}
