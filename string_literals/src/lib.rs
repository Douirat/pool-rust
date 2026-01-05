pub fn is_empty(v: &str) -> bool {
v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
if v == "" {
    return true
}
if pat.len() > v.len() {
    return false
}
for i in 0..(v.len()-pat.len()){
println!("{}", &v[i..i+pat.len()]);
    if &v[i..i+pat.len()] == pat{
        return true
    }
}
false
}


pub fn split_at(v: &str, index: usize) -> (&str, &str) {
let str1 = &v[0..index];
let str2 = &v[index..v.len()];
(str1, str2)
}

pub fn find(v: &str, pat: char) -> usize {
let mut x: usize = 0;
for i in v.chars(){
    if i == pat {
        break
    } 
    x += 1;
}
x
}
