fn swap(a: f32, b: f32) -> [f32; 2] {
	return [b, a];
}

fn quicksort(n: usize, arr: &mut [f32]) {
	const M: i32 = 7;
	const N_STACK: i32 = 50;
	
	let i: usize;
	let ir = n;
	let j: usize;
	let k: usize;
	let l = 1;
	let *istack: usize;
	let jstack: i32 = 0;
	let a: f32;
	let temp: f32;
	
	//istack = lvec
}

fn main() {
    let mut test_array: [f32; 3] = [4.2, 2.3, 1.2];
	quicksort(3, &mut test_array);
	let _k: i32;
	for _k in 0..3 {
		println!("{}, ", test_array[_k]);
	}
}
