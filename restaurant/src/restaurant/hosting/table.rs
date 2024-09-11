use crate::Customer;

pub struct Table {
    pub number: u32,
    pub capacity: u32,
    pub guests: Vec<Customer>,
    pub is_available: bool,
}

impl Table {
    pub fn new(number: u32, capacity: u32) -> Table {
        Table {
            number,
            capacity,
            guests: Vec::new(),
            is_available: true,
        }
    }

    pub fn seat(&mut self, customer: Customer) {
        if self.capacity <= 0 {
            return;
        }

        self.capacity -= 1;
        self.is_available = self.capacity > 0;
        self.guests.push(customer);
    }

    pub fn release(&mut self) {
        self.capacity = self.guests.len() as u32;
        self.is_available = true;
        self.guests.clear();
    }
}
