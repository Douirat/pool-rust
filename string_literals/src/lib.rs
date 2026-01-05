pub fn is_empty(v: &str) -> bool {
v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
v.contains(pat)
}


pub fn split_at(v: &str, index: usize) -> (&str, &str) {
let str1 = &v[0..index];
let str2 = &v[index..v.len()];
(str1, str2)
}

pub fn find(v: &str, pat: char) -> usize {
some(v.find(pat))
}
