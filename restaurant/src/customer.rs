use crate::restaurant::serving::menu::item::MenuItem;

/// The Customer struct represents a customer with a name and an order.
pub struct Customer {
    /// The name of the customer.
    pub name: String,
    /// The order of the customer.
    pub order: Vec<MenuItem>,
}

impl Customer {
    /// This function creates a new customer with the given name.
    pub fn new(name: &str) -> Customer {
        Customer {
            name: name.to_string(),
            order: Vec::new(),
        }
    }

    /// This function adds an item to the order of the customer.
    pub fn add_to_order(&mut self, item: MenuItem) {
        self.order.push(item);
    }

    /// This function calculates the total amount to be paid by the customer.
    pub fn order_total(&self) -> f64 {
        self.order.iter().map(|item| item.price).sum()
    }
}

impl Clone for Customer {
    /// This function is used to clone a customer (implementing the Clone trait).
    fn clone(&self) -> Customer {
        Customer {
            name: self.name.clone(),
            order: self.order.clone(),
        }
    }
}
