/// The MenuItem struct represents a menu item with a name and a price.
pub struct MenuItem {
    /// The name of the menu item.
    pub name: String,
    /// The price of the menu item.
    pub price: f64,
}

impl MenuItem {
    /// This function creates a new menu item with the given name and price.
    pub fn new(name: &str, price: f64) -> MenuItem {
        MenuItem {
            name: name.to_string(),
            price,
        }
    }
}

impl Clone for MenuItem {
    /// This function is used to clone a menu item (implementing the Clone trait).
    fn clone(&self) -> MenuItem {
        MenuItem {
            name: self.name.clone(),
            price: self.price,
        }
    }
}
