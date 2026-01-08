use std::collections::HashMap;
 use std::iter::FromIterator;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut result = i32::MIN;
    for (_, value) in h.iter(){
        if *value > result{
              result = *value;
        }
      
    }
    // *h.values().max().unwrap_or(&i32::MAX)
   result
}