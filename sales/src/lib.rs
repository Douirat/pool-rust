#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some((name, price)) = s.products.iter().find(|p| p.0 == ele) {
            self.items.push((name.clone(), *price));
        }
    }

pub fn generate_receipt(&mut self) -> Vec<f32> {
    let mut result = Vec::new();

    // process in insertion order in groups of 3
    self.items
        .chunks(3)
        .for_each(|chunk| {
            if chunk.len() == 3 {
                // sum and min for this group
                let sum: f32 = chunk.iter().map(|p| p.1).sum();
                let min = chunk.iter().map(|p| p.1).fold(f32::MAX, f32::min);

                let factor = (sum - min) / sum;

                // apply discount proportionally
                chunk.iter().for_each(|p| {
                    let v = (p.1 * factor * 100.0).round() / 100.0;
                    result.push(v);
                });
            } else {
                // leftover items, no discount
                chunk.iter().for_each(|p| {
                    let v = (p.1 * 100.0).round() / 100.0;
                    result.push(v);
                });
            }
        });

    // sort final receipt
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());

    self.receipt = result.clone();
    result
}



}
