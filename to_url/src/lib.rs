pub fn to_url(s: &str) -> String {
    let mut result: String = String::new();
    let pud = &"%20";
    for i in s.chars() {
        if i.is_whitespace(){
            result += pud;
            continue
        }
        result.push(i);
    }
    result
}
