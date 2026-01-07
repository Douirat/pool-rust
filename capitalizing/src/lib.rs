pub fn capitalize_first(input: &str) -> String {
 let mut result = String::new();
 let mut start = true;
 for c in input.chars(){
    if start{
        let r = c.to_uppercase().to_string();
        result += &r;
        start = false;
        continue
    }
result.push(c);
 }
result
}


pub fn title_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            // preserve all whitespace
            capitalize_next = true;
            result.push(c);
        } else if capitalize_next {
            result.extend(c.to_uppercase()); // capitalize first char of the word
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}
