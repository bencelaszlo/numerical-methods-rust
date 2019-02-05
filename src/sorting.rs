fn heapify(a: &mut [f32], n: usize, i: usize) {
	let mut largest: usize = i;
	let l = 2 * i + 1;
	let r = 2 * i + 2;
	
	if r < n && a[i] < a[l] {
		largest = l;
	}
	
	if r < n && a[largest] < a[r] {
		largest = r;
	}
	
	if largest != i {
		let temp = a[largest];
		a[largest] = a[i];
		a[i] = temp;
		
		heapify(a, n, largest);
	}
}

pub fn heapsort(n: usize, a: &mut [f32]) {
	//Build a maxheap
	let mut i: usize = n;
	while i > 0 {
		heapify(a, n, i);
		i -= 1;
	}

	//#One by one extract elements
	i = n -1;
	while i > 0 {
		let temp = a[0];
		a[0] = a[i];
		a[i] = temp;
		heapify(a, i, 0);
		i -= 1;
	}
}

// ~N^1.25 routine
// Sorts an array arr[1..n] into ascending numerical order, by Shell's method. n is input; arr is replaced on output by its sorted rearrangement
pub fn shell_sort(n: i32, a: &mut [f32]) {
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

//Hoare's Partition Schema
pub fn quicksort(low: i32, high: i32, arr: &mut [f32]) {
	if low < high {
		let p: i32 = partition(low, high, arr);
		quicksort(low, p, arr);
		quicksort(p+1, high, arr);
		return;
	}
}

fn partition(low: i32, high: i32, arr: &mut [f32]) -> i32 {
	let pivot: f32;
	pivot  = arr[( (low + high) / 2) as usize];
	let mut i: i32;
	let mut j: i32;

	i = low;
	j = high;
    println!("i: {}, j: {}", i, j);

	loop {
		while arr[i as usize] < pivot {
			i += 1;
		}

		while arr[j as usize] > pivot {
			j -= 1;
		}

		if i >= j {
			return j as i32;
		}

		//SWAP
		arr[i as usize] = arr[i as usize] + arr[j as usize];
		arr[j as usize] = arr[i as usize] - arr[j as usize];
		arr[i as usize] = arr[i as usize] - arr[j as usize];
	}
}