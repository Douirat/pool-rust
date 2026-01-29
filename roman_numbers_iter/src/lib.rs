#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RomanDigit {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl RomanNumber {
    pub fn from(num: u32) -> Self {
        let mut result = Vec::new();
        let mut n = num;

        while n >= 1000 {
            result.push(RomanDigit::M);
            n -= 1000;
        }
        while n >= 900 {
            result.push(RomanDigit::C);
            result.push(RomanDigit::M);
            n -= 900;
        }
        while n >= 500 {
            result.push(RomanDigit::D);
            n -= 500;
        }
        while n >= 400 {
            result.push(RomanDigit::C);
            result.push(RomanDigit::D);
            n -= 400;
        }
        while n >= 100 {
            result.push(RomanDigit::C);
            n -= 100;
        }
        while n >= 90 {
            result.push(RomanDigit::X);
            result.push(RomanDigit::C);
            n -= 90;
        }
        while n >= 50 {
            result.push(RomanDigit::L);
            n -= 50;
        }
        while n >= 40 {
            result.push(RomanDigit::X);
            result.push(RomanDigit::L);
            n -= 40;
        }
        while n >= 10 {
            result.push(RomanDigit::X);
            n -= 10;
        }
        while n >= 9 {
            result.push(RomanDigit::I);
            result.push(RomanDigit::X);
            n -= 9;
        }
        while n >= 5 {
            result.push(RomanDigit::V);
            n -= 5;
        }
        while n >= 4 {
            result.push(RomanDigit::I);
            result.push(RomanDigit::V);
            n -= 4;
        }
        while n >= 1 {
            result.push(RomanDigit::I);
            n -= 1;
        }

        RomanNumber(result)
    }

    fn value(&self) -> u32 {
        let mut total = 0;
        let mut prev = 0;

        for digit in self.0.iter().rev() {
            let val = *digit as u32;
            if val < prev {
                total -= val;
            } else {
                total += val;
                prev = val;
            }
        }

        total
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.value() + 1;
        let next = RomanNumber::from(next_value);
        self.0 = next.0.clone();
        Some(next)
    }
}