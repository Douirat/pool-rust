use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut m:HashMap<char, bool> = HashMap::new();
        for c in 'a'..='z'{
        m.insert(c, false);
    }

     for c in s.to_lowercase().chars(){
      if ('a'..='z').contains(&c) {
        let v = m.entry(c).or_insert(false);
        *v = true;
      }
    }
//   println!("{:?}", m);
    for (k, v) in &m{
        //  println!("--> {} {}", k, v);
        if *v != true{
            return false
        }
    }
true
}
