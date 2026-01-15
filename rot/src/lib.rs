pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();
    let key = ((key % 26) + 26) % 26;
    for c in input.chars(){
        let x: char = match c {
            'a'..='z' => ((((c as u8 - b'a') + key.abs() as u8) %26) + b'a') as char,
            'A'..='Z' =>((((c as u8 - b'A') + key.abs() as u8) % 26) + b'A') as char,
            _ => c,
         };
         result.push(x);
    }
    result
}