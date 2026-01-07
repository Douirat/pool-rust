pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32] // array of 32 tens
}

pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum() // sum elements of the slice
}
