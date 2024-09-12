pub mod menu;

use super::hosting::iter_tables;
pub use menu::extend_menu;
use menu::{ask_for_order, get_random_order};

/// This function serves the orders for all the tables.
/// It prints the orders for each customer at each table.
pub fn serve_orders() {
    let tables = iter_tables();
    println!("Serving orders:\n");
    println!("{:12}{:12}{}", "Customer", "Table No.", "Order");
    for table in tables.iter() {
        for customer in table.guests.iter() {
            println!(
                "{:12}{:02}{:10}{}",
                customer.name,
                table.number,
                "",
                customer
                    .order
                    .iter()
                    .map(|item| item.name.clone())
                    .collect::<Vec<String>>()
                    .join(", "),
            );
        }
    }
    println!("");
}

/// This function takes orders for all the customers at all the tables.
/// It assigns can either ask for orders or generate random orders.
pub fn take_orders(random: bool) {
    let mut tables = iter_tables();
    for table in tables.iter_mut() {
        for customer in table.guests.iter_mut() {
            if random {
                get_random_order(customer);
            } else {
                ask_for_order(customer);
            }
        }
    }
}

/// This function takes payments for all the customers at all the tables.
/// It prints the total amount to be paid by each customer.
/// It also releases the tables after taking payments.
/// It returns the total amount collected.
pub fn take_payments() -> f64 {
    let mut tables = iter_tables();
    let mut sum = 0.0;

    println!("Taking payments:\n");
    println!("{:12}{:12}{}", "Customer", "Table No.", "Total");
    for table in tables.iter_mut() {
        for customer in table.guests.iter() {
            let total = customer.order_total();
            println!(
                "{:12}{:02}{:10}${:.2}",
                customer.name, table.number, "", total,
            );
            sum += total;
        }
        table.release();
    }
    println!("");

    sum
}
