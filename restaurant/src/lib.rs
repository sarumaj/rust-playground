pub mod customer;
pub mod restaurant;

pub use customer::Customer;
pub use restaurant::hosting::{add_table, add_to_wait_list, process_wait_list};
pub use restaurant::serving::{extend_menu, serve_orders, take_orders, take_payments};
