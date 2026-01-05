use std::io::{stdin,stdout,Write};


fn main() {
    let answer = "The letter e";
    let mut trails: i32 = 0;
loop{
    trails += 1;
    let mut s=String::new();
    print!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?\n");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    if s == answer {
    println!("Number of trials: {}", trails);
    break
    }
}
}