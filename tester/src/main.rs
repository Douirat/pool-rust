use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let path = path.as_ref();
    let mut existing_content = String::new();

    if let Ok(mut file) = fs::File::open(path) {
        file.read_to_string(&mut existing_content)
            .expect("Cannot read file");
    }

    existing_content.push_str(content);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Cannot open file for writing");

    file.write_all(existing_content.as_bytes())
        .expect("Cannot write to file");
}




fn main() {
    let path = "a.txt";
    
    open_or_create(&path, "content to be written");

    let contents = fs::read_to_string(path).unwrap();
    println!("{}", contents);
}




