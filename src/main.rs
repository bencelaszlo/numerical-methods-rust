mod utils;
mod sorting;
mod integration;
mod interpolation;

fn main() {
    let mut test_numbers: Vec<f32> = Vec::new();
    let _file_read_result = utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);
    println!("SORTING");
	for k in 0..test_numbers.len() {
		print!("{}, ", test_numbers[k]);
	}

    let mut test_numbers: Vec<f32> = Vec::new();
    let _file_read_result = utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);
    println!("\n\nShell Sort");
    sorting::shell_sort(test_numbers.len() as i32, &mut test_numbers);
	for k in 0..test_numbers.len() {
		print!("{}, ", test_numbers[k]);
	}
    
    let mut test_numbers: Vec<f32> = Vec::new();
    let _file_read_result = utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);
    println!("\n\nHeap Sort");
    sorting::heapsort(test_numbers.len(), &mut test_numbers);
    for k in 0..test_numbers.len() {
		print!("{}, ", test_numbers[k]);
	}

    let mut test_numbers: Vec<f32> = Vec::new();
    let _file_read_result = utils::read_numbers_from_file("input.txt".to_string(), &mut test_numbers);
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
    let mut y: Vec<f32> = x.clone();
    interpolation::linear_interpolation_array(&x_known, &y_known, test_numbers, &mut y);
    for k in 0..y.len() {
        print!("{}\n", y[k]);
    }

    println!("\n\n\n");

    println!("INTEGRATION OF FUNCTIONS");

    println!("\n\nTrapedozial Rule");
	println!("{}", integration::q_trapedozial_rule(0.0, 3000.0) );

    println!("\n\nSimpson's Rule");
	println!("{}", integration::q_simpsons_rule(0.0, 3000.0) );

    println!("\n\nGauss-Legendre");
    println!("{}", integration::q_gauss_legendre(0.0, 3000.0) );
}