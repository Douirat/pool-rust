pub fn rev_str(input: &str) -> String {
    let mut s: String = String::new();
    for i in input.chars().rev(){
        s.push(i)
    }
    s
}