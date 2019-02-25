mod utils;
mod sorting;
mod integration;
mod interpolation;

fn main() {
    println!("SORTING");
    let mut test_numbers = vec![6.4, -0.21, 3.1415926, 4.2, 2.225, 2.3, 1.2, 2.4, 5.2];
	for k in 0..test_numbers.len() {
		print!("{}, ", test_numbers[k]);
	}

    println!("\n\nShell Sort");
    test_numbers = vec![6.4, -0.21, 3.1415926, 4.2, 2.225, 2.3, 1.2, 2.4, 5.2];
    sorting::shell_sort(test_numbers.len() as i32, &mut test_numbers);
	for k in 0..test_numbers.len() {
		print!("{}, ", test_numbers[k]);
	}
    
    println!("\n\nHeap Sort");
    test_numbers = vec![6.4, -0.21, 3.1415926, 4.2, 2.225, 2.3, 1.2, 2.4, 5.2];
    sorting::heapsort(test_numbers.len(), &mut test_numbers);
    for k in 0..test_numbers.len() {
		print!("{}, ", test_numbers[k]);
	}

    println!("\n\nQuicksort");
    test_numbers = vec![6.4, -0.21, 3.1415926, 4.2, 2.225, 2.3, 1.2, 2.4, 5.2];
    sorting::quicksort(0, (test_numbers.len() - 1) as i32, &mut test_numbers);
    for k in 0..test_numbers.len() {
		print!("{}, ", test_numbers[k]);
	}

    println!("\n\n\n");

    println!("INTERPOLATION");

    println!("\n\nLinear Interpolation");
    let x_known: [f32; 7] = [-250.0, -200.0, -100.0, 0.0, 100.0, 200.0, 250.0];
    let y_known: [f32; 7] = [0.0, 280.0, 540.0, 640.0, 1200.0, 1600.0, 1800.0];
    let x: Vec<f32> = utils::read_numbers_from_file("input.txt".to_string() );
    let mut y: Vec<f32> = x.clone();
    interpolation::linear_interpolation_array(&x_known, &y_known, x, &mut y);
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