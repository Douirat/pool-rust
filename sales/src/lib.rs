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
    let prices: Vec<f32> = self.items.iter().map(|p| p.1).collect();

    if prices.len() < 3 {
        self.receipt = prices.clone();
        return prices;
    }

    let sum: f32 = prices.iter().sum();
    let min = prices
        .iter()
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let factor = (sum - min) / sum;

    let mut result: Vec<f32> = prices
        .iter()
        .map(|p| ((p * factor) * 100.0).round() / 100.0)
        .collect();

    result.sort_by(|a, b| a.partial_cmp(b).unwrap());

    self.receipt = result.clone();
    result
}


}
