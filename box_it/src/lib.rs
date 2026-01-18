pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace().map(|w| {
        let has_k = w.ends_with('k') || w.ends_with('k');
        let v = if has_k {&w[0..w.len()-1]} else{w};
        let p = v.parse::<f64>().unwrap();
        let i = if has_k{p * 1000.0} else {p};
        Box::new(i as u32)
    })
    .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    for i in a {
        let n: u32 = *i;
        v.push(n);
    }
    v
}