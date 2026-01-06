pub fn nbr_function(c: i32) -> (i32, f64, f64) {
(c, (c.clone() as f64).exp() ,(c.clone() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
let s = a.clone();
let mut result = String::new();
for (i, c)in s.chars().enumerate(){
if c == ' ' {
    continue
}
   let int = (((c as u32) as i32) - 48) as f64;
     result += &int.ln().to_string();
     if i < s.len(){
         result.push(' ');
     }
}


(a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
let mut b1 = Vec::new();
for i in b.clone() {
    b1.push((i as f64).ln());
   
}
(b, b1)
}
