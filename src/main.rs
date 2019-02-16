mod sorting;
mod integration;
mod interpolation;

fn main() {
    println!("SORTING");
    let mut test_array: [f32; 9] = [6.4, -0.21, 3.1415926, 4.2, 2.225, 2.3, 1.2, 2.4, 5.2];
    let _k: i32;
	for _k in 0..test_array.len() {
		print!("{}, ", test_array[_k]);
	}

    println!("\n\nShell Sort");
    sorting::shell_sort(test_array.len() as i32, &mut test_array);
    let _k: i32;
	for _k in 0..test_array.len() {
		print!("{}, ", test_array[_k]);
	}
    
    println!("\n\nHeap Sort");
    test_array = [6.4, -0.21, 3.1415926, 4.2, 2.225, 2.3, 1.2, 2.4, 5.2];
    sorting::heapsort(test_array.len(), &mut test_array);
    for _k in 0..test_array.len() {
		print!("{}, ", test_array[_k]);
	}

    println!("\n\nQuicksort");
    test_array = [6.4, -0.21, 3.1415926, 4.2, 2.225, 2.3, 1.2, 2.4, 5.2];
    sorting::quicksort(0, (test_array.len() - 1) as i32, &mut test_array);
    for _k in 0..test_array.len() {
		print!("{}, ", test_array[_k]);
	}

    println!("\n\n\n");

    println!("INTERPOLATION");

    println!("\n\nLinear Interpolation");
    let x_known: [f32; 3] = [0.0, 1.0, 2.0];
    let y_known: [f32; 3] = [1.0, 4.0, 5.0];
    let x: [f32; 3] = [0.5, 1.25, 1.75];
    let mut y: [f32; 3] = [0.0, 0.0, 0.0];
    interpolation::linear_interpolation_array(&x_known, &y_known, &x, &mut y);
    for k in 0..y.len() {
        print!("{}, ", y[k]);
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