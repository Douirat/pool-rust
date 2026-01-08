use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut result = i32::MAX;
    for (_, value) in *h{
        if value < result;
        result = value;
    }
    // *h.values().max().unwrap_or(&i32::MAX)
    value
}