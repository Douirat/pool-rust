pub fn stars(n: u32) -> String {
    let mut x:usize = 2;
    
    for i in 1..n{
        x *= 2
    }
   
"*".to_string().repeat(x)
}