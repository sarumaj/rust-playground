//! # Restaurant
//!
//! `restaurant` is a collection of utilities to help manage a restaurant.
//!
//! ## Features
//!
//! - Add customers to a wait list.
//! - Add tables to the restaurant.
//! - Take orders for customers.
//! - Serve orders to customers.
//! - Take payments from customers.
//! - Process the wait list and seat customers at available tables.
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! restaurant = "0.1"
//! ```
//!
//! Then, add the following to your Rust code:
//!
//! ```rust
//! use restaurant::*;
//!
//! fn main() {
//!    let customers = vec![
//!       Customer::new("Alice"),
//!       Customer::new("Bob"),
//!       Customer::new("Carol"),
//!       Customer::new("Dave"),
//!       Customer::new("Eve"),
//!       Customer::new("Frank"),
//!    ];
//!
//!   extend_menu("Noodles", 16.39);
//!   extend_menu("Water", 0.89);
//!
//!   for customer in customers {
//!      add_to_wait_list(customer);
//!   }
//!
//!   add_table(1);
//!
//!   let mut revenue = 0.0;
//!   loop {
//!      process_wait_list();
//!      take_orders(true);
//!      serve_orders();
//!
//!      let daily_revenue = take_payments();
//!      if daily_revenue == 0.0 {
//!         break;
//!      }
//!
//!      revenue += daily_revenue;
//!   }
//!   println!("Total revenue: ${:.2}.", revenue);
//! }
//! ```

pub mod customer;
pub mod restaurant;

pub use customer::Customer;
pub use restaurant::hosting::{add_table, add_to_wait_list, process_wait_list};
pub use restaurant::serving::{extend_menu, serve_orders, take_orders, take_payments};
