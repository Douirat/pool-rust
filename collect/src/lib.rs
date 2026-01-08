pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return; // already sorted
    }

    // Outer loop: each pass
    for i in 0..n {
        let mut swapped = false;

        // Inner loop: compare adjacent elements
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1); // swap if out of order
                swapped = true;
            }
        }

        // If no swaps in a pass → already sorted
        if !swapped {
            break;
        }
    }
}
