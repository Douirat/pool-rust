pub fn stars(n: u32) -> String {
    let mut x:usize = 2;
    x.pow(n);
"*".to_string().repeat(x)
}