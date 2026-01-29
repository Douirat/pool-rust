sales#[derive(Debug, Clone, PartialEq)]
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
        // extract prices
        let mut prices: Vec<f32> = self.items.iter().map(|p| p.1).collect();

        // sort ascending
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut result: Vec<f32> = Vec::new();

        // apply promotion in groups of 3
        prices.chunks(3).for_each(|chunk| {
            if chunk.len() == 3 {
                let min = chunk[0];
                let discount = min / 3.0;

                chunk.iter().for_each(|p| {
                    let v = ((p - discount) * 100.0).round() / 100.0;
                    result.push(v);
                });
            } else {
                // leftover items (no promotion)
                chunk.iter().for_each(|p| {
                    let v = (*p * 100.0).round() / 100.0;
                    result.push(v);
                });
            }
        });

        // final sorting
        result.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt = result.clone();
        result
    }
}
