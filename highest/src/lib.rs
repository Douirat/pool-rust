#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self{
            numbers
        }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.is_empty() {
            return None;
        } 
        Some(self.numbers[self.numbers.len() -1])
    }

    pub fn highest(&self) -> Option<u32> {
        if self.numbers.is_empty() {
            return None;
        } 
        let mut max = self.numbers[0];
        for i in self.numbers {
            max = max.max(*i)
        }
       Some(max)
    }

pub fn highest_three(&self) -> Vec<u32> {
    let mut result = Vec::new();
    let mut c = self.numbers.to_vec(); // make mutable copy

    for _ in 0..3 {
        if c.is_empty() {
            break; // less than 3 numbers
        }
        // Find the max
        let mut max_index = 0;
        for i in 1..c.len() {
            if c[i] > c[max_index] {
                max_index = i;
            }
        }
        // Push max to result
        result.push(c[max_index]);
        // Remove it from c
        c.remove(max_index);
    }

    result
}

}
