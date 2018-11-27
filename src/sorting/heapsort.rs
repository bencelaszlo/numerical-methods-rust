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

fn heapsort(n: usize, a: &mut [f32]) {
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

fn main() {
    let mut test_array: [f32; 5] = [6.4, 1.3, 4.2, 2.3, 1.2];
	heapsort(5, &mut test_array);
	for k in 0..5 {
		println!("{}, ", test_array[k]);
	}
}