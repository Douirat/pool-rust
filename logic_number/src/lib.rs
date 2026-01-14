pub fn number_logic(num: u32) -> bool {
    let mut units:Vec<u32> = Vec::new();
    let mut x = num;
    let mut result: u32 = 0;
   while x > 0 {
    units.push(x %10);
    x/=10;
   }
   let mut u = units.len() as u32;
   for i in units{
    result += i.pow(u);
   }
    num == result
}
