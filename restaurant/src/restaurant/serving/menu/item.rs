pub struct MenuItem {
    pub name: String,
    pub price: f64,
}

impl MenuItem {
    pub fn new(name: &str, price: f64) -> MenuItem {
        MenuItem {
            name: name.to_string(),
            price,
        }
    }
}

impl Clone for MenuItem {
    fn clone(&self) -> MenuItem {
        MenuItem {
            name: self.name.clone(),
            price: self.price,
        }
    }
}
