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
        // Chercher le produit dans le store
        for (name, price) in &s.products {
            if name == &ele {
                self.items.push((ele.clone(), *price));
                break;
            }
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let free_items = prices.len() / 3;

        let total_discount: f32 = prices.iter().take(free_items).sum();

        let total_price: f32 = prices.iter().sum();

        let final_price = total_price - total_discount;

        let ratio = if total_price > 0.0 {
            final_price / total_price
        } else {
            1.0
        };

        let receipt: Vec<f32> = prices
            .iter()
            .map(|price| (price * ratio * 100.0).round() / 100.0)
            .collect();

        self.receipt = receipt.clone();

        receipt
    }
}