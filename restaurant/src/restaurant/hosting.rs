pub mod table;

use crate::Customer;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use table::Table;

static QUEUE: Lazy<Mutex<Vec<Customer>>> = Lazy::new(|| Mutex::new(Vec::new()));

static TABLES: Lazy<Mutex<Vec<Table>>> = Lazy::new(|| {
    let mut tables = Vec::new();
    tables.push(Table::new(1, 4));
    tables.push(Table::new(2, 2));
    tables.push(Table::new(3, 2));
    tables.push(Table::new(4, 3));
    tables.push(Table::new(5, 3));
    Mutex::new(tables)
});

pub fn add_to_wait_list(customer: Customer) {
    let mut queue = QUEUE.lock().unwrap();
    queue.push(customer);
}

pub fn add_table(capacity: u32) {
    let mut tables = TABLES.lock().unwrap();
    let table_number = match tables.last() {
        Some(table) => table.number+1,
        None => 1,
    };
    tables.push(Table::new(table_number, capacity));
}

pub fn iter_tables() -> std::sync::MutexGuard<'static, Vec<Table>> {
    TABLES.lock().unwrap()
}

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
