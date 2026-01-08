pub fn bubble_sort(arr: &mut [i32]) {
for _i in 0..arr.len()-1{
    for j in 1..arr.len() {
        if arr[j] < arr[j-1] {
            arr[j] = arr[j] + arr[j-1];
            arr[j-1] = arr[j] - arr[j-1];
            arr[j] = arr[j] - arr[j-1];
        }
    }
}
}
