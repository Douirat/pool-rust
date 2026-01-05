pub fn is_empty(v: &str) -> bool {
v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
for i in v.chars() {
    if !i.is_ascii(){
        return false
    }
}
true
}

pub fn contains(v: &str, pat: &str) -> bool {
if pat.len() > v.len(){
    return false
}
for i in 0..(v.len()-pat.len()){
    if &v[i..i+pat.len()] == pat {
        return true
    }
}
false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
(&v[0..index], &v[index..v.len()])
}

pub fn find(v: &str, pat: char) -> usize {
for (i, c) in v.chars().enumerate() {
    if c == pat{
        return i;
    }
}
usize::MAX
}