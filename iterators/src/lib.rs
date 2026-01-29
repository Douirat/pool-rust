#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
    finished: bool,
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { v: n, finished: n == 0 }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let current = self.v;

        if self.v == 1 {
            self.finished = true;
        } else if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = 3 * self.v + 1;
        }

        Some(current)
    }
}