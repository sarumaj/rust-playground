pub mod table;

use crate::Customer;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use table::Table;

/// This is a static, yet expandable vector that holds the list of customers waiting to be seated.
static QUEUE: Lazy<Mutex<Vec<Customer>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// This is a static, yet expandable vector that holds the list of tables in the restaurant.
static TABLES: Lazy<Mutex<Vec<Table>>> = Lazy::new(|| {
    let mut tables = Vec::new();
    tables.push(Table::new(1, 4));
    tables.push(Table::new(2, 2));
    tables.push(Table::new(3, 2));
    tables.push(Table::new(4, 3));
    tables.push(Table::new(5, 3));
    Mutex::new(tables)
});

/// This function is used to add a customer to the wait list.
pub fn add_to_wait_list(customer: Customer) {
    let mut queue = QUEUE.lock().unwrap();
    queue.push(customer);
}

/// This function is used to add a table to the restaurant.
/// It takes the capacity of the table as an argument.
pub fn add_table(capacity: u32) {
    let mut tables = TABLES.lock().unwrap();
    let table_number = match tables.last() {
        Some(table) => table.number + 1,
        None => 1,
    };
    tables.push(Table::new(table_number, capacity));
}

/// This function is used to get a reference to the list of tables.
pub fn iter_tables() -> std::sync::MutexGuard<'static, Vec<Table>> {
    TABLES.lock().unwrap()
}

/// This function is used to process the wait list and seat customers at available tables.
/// It assigns customers to tables based on availability and removes them from the wait list.
/// It prints the assigned tables and customers.
pub fn process_wait_list() {
    let mut queue = QUEUE.lock().unwrap();
    let mut tables = TABLES.lock().unwrap();

    println!("Processing guest list:\n");
    println!("{:12}{}", "Customer", "Table No.");
    for customer in queue.iter() {
        for table in tables.iter_mut() {
            if table.is_available {
                println!("{:12}{:02}", customer.name, table.number);
                table.seat(customer.clone());
                break;
            }
        }
    }
    println!("");

    for table in tables.iter() {
        for customer in table.guests.iter() {
            queue.retain(|c| c.name != customer.name);
        }
    }
}
