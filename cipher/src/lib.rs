#[derive(Debug, PartialEq)]
pub struct CipherError {
   expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
   let mut s = String::new();
  
   for c in original.chars() {
       if c.is_ascii_lowercase(){
           let temp = (b'z' + b'a' - c as u8) as char;
             s.push(temp);
       } else if c.is_ascii_uppercase(){
           let temp =( b'Z' + b'A' - c as u8) as char;
           s.push(temp);
       } else {
           s.push(c)
       }
   }
 
   if s == ciphered.to_string() {
        Ok(())
   } else {
        Err(CipherError{expected: s})
   }
}