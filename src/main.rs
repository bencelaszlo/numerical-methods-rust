mod sorting;

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
}