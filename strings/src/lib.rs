pub fn char_length(s: &str) -> usize {
    let mut count: usize = 0;
    for _i in s {
        count +=1;
    }
    count
}