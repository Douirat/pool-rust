ub fn rev_str(input: &str) -> String {
    let mut s: String = STring::new();
    for i in input.chars().rev(){
        s.push(i)
    }
    s
}