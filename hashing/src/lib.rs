
use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
let mut arr:Vec<i32> = Vec::new();
for i in list.iter(){
    arr.push(*i);
}
bubble_sort(&mut arr);
println!("{:?}", arr);
if arr.len() % 2 == 0{
   (arr[arr.len() / 2 as usize] +  arr[(arr.len() / 2 as usize) + 1] ) / 2
} else{
     arr[arr.len() / 2 as usize]
}
}


fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut freq = HashMap::new();

    for &n in list {
        *freq.entry(n).or_insert(0) += 1;
    }

    let mut max_value = list[0];
    let mut max_count = 0;

    for (&value, &count) in freq.iter() {
        if count > max_count {
            max_count = count;
            max_value = value;
        }
    }

    max_value
}
