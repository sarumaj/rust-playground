use crate::restaurant::serving::menu::item::MenuItem;

pub struct Customer {
    pub name: String,
    pub order: Vec<MenuItem>,
}

impl Customer {
    pub fn new(name: &str) -> Customer {
        Customer {
            name: name.to_string(),
            order: Vec::new(),
        }
    }

    pub fn add_to_order(&mut self, item: MenuItem) {
        self.order.push(item);
    }

    pub fn order_total(&self) -> f64 {
        self.order.iter().map(|item| item.price).sum()
    }
}

impl Clone for Customer {
    fn clone(&self) -> Customer {
        Customer {
            name: self.name.clone(),
            order: self.order.clone(),
        }
    }
}
