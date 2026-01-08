pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut v = list.to_vec();
    v.sort();

    let mid = v.len() / 2;

    if v.len() % 2 == 1 {
        v[mid]
    } else {
        (v[mid - 1] + v[mid]) / 2
    }
}

use std::collections::HashMap;

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
