// ~N^1.25 routine
// Sorts an array arr[1..n] into ascending numerical order, by Shell's method. n is input; arr is replaced on output by its sorted rearrangement
fn shell_sort(n: i32, a: &mut [f32]) {
	let mut gap: i32 =  (n / 2) as i32;
	let mut temp: f32;
	let mut j: i32;
	
	while gap > 0 {
		for i in gap..n {
			temp = a[i as usize];
			
			j = i;
			while j >= gap && a[(j-gap) as usize] > temp {
				a[j as usize] = a[(j-gap) as usize];
				j -= gap;
			}
			a[j as usize] = temp;
		}
		gap /= 2;
	}
}

fn main() {
    let mut test_array: [f32; 5] = [6.4, 3.1415926, 4.2, 2.3, 1.2];
	shell_sort(5, &mut test_array);
	let _k: i32;
	for _k in 0..5 {
		println!("{}, ", test_array[_k]);
	}
}

