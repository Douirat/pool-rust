pub fn stars(n: u32) -> String {
    let mut x:usize = 2;
    if n == 0 {
        x = 1;
    }
    for i in 1..n{
        x *= 2;
    }
   
"*".to_string().repeat(x)
}