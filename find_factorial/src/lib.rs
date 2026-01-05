pub fn factorial(num: u64) -> u64 {
let mut steps: u64 = 1;
let mut product:u64 = 1;
while steps <= num {
    product *= steps;
    steps += 1;
}
product
}