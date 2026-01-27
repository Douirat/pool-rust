#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u32,
}

impl Person {
    pub fn new<'a>(name: &'a str) -> Self {
        Self{
            name,
            age: 0,
        }
    }
}