fn heapify(a: &mut Vec<f32>, n: usize, i: usize) {
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
        a.swap(largest, i);

        heapify(a, n, largest);
    }
}

pub fn heapsort(n: usize, a: &mut Vec<f32>) {
    //Build a maxheap
    let mut i: usize = n;
    while i > 0 {
        heapify(a, n, i);
        i -= 1;
    }

    //#One by one extract elements
    i = n - 1;
    while i > 0 {
        a.swap(0, i);
        heapify(a, i, 0);
        i -= 1;
    }
}

// Sorts an array arr[1..n] into ascending numerical order, by Shell's method. n is input; arr is replaced on output by its sorted rearrangement
pub fn shell_sort(n: i32, a: &mut Vec<f32>) {
    let mut gap: i32 = (n / 2) as i32;
    let mut temp: f32;
    let mut j: i32;

    while gap > 0 {
        for i in gap..n {
            temp = a[i as usize];

            j = i;
            while j >= gap && a[(j - gap) as usize] > temp {
                a[j as usize] = a[(j - gap) as usize];
                j -= gap;
            }
            a[j as usize] = temp;
        }
        gap /= 2;
    }
}

//Hoare's Partition Schema
pub fn quicksort(low: i32, high: i32, arr: &mut Vec<f32>) {
    if low < high {
        let p: i32 = partition(low, high, arr);
        quicksort(low, p, arr);
        quicksort(p + 1, high, arr);
    } else {
        return;
    }
}

fn partition(low: i32, high: i32, arr: &mut Vec<f32>) -> i32 {
    let pivot: f32;

    // Choose pivot element with the "median-of-three" rule
    let mid: i32 = (low + high) / 2;
    if arr[mid as usize] < arr[low as usize] {
        arr.swap(low as usize, high as usize);
    }
    if arr[high as usize] < arr[low as usize] {
        arr.swap(low as usize, high as usize);
    }
    if arr[mid as usize] < arr[high as usize] {
        arr.swap(mid as usize, high as usize);
    }
    pivot = arr[high as usize];

    let mut i: i32;
    let mut j: i32;

    i = low - 1;
    j = high + 1;

    loop {
        loop {
            i += 1;
            if arr[i as usize] >= pivot {
                break;
            }
        }

        loop {
            j -= 1;
            if arr[j as usize] <= pivot {
                break;
            }
        }

        if i >= j {
            return j as i32;
        }

        //SWAP
        arr.swap(i as usize, j as usize);
    }
}
