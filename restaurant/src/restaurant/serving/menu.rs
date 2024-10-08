pub mod item;

use crate::Customer;
use item::MenuItem;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

/// This is a static, yet expandable list of items on the menu.
static MENU_LIST: Lazy<Mutex<HashMap<&'static str, MenuItem>>> = Lazy::new(|| {
    let mut menu = HashMap::new();
    menu.insert("Ramen", MenuItem::new("Ramen", 12.99));
    menu.insert("Sushi", MenuItem::new("Sushi", 8.79));
    menu.insert("Curry", MenuItem::new("Curry", 10.29));
    menu.insert("Pizza", MenuItem::new("Pizza", 14.99));
    menu.insert("Sake", MenuItem::new("Sake", 5.49));
    menu.insert("Cha", MenuItem::new("Cha", 2.29));
    menu.insert("Black Tea", MenuItem::new("Black Tea", 3.39));
    menu.insert("Sapporo", MenuItem::new("Sapporo", 4.49));
    Mutex::new(menu)
});

/// This function is used to ask a customer for their order.
/// It prints the menu and asks the customer to enter the items they want to order.
/// It assigns the order to the customer.
pub fn ask_for_order(customer: &mut Customer) {
    let menu = MENU_LIST.lock().unwrap();
    let mut order = Vec::new();

    println!("What would you like to order, {}?", customer.name);
    for (name, item) in menu.iter() {
        println!("{:12} - ${}", name, item.price);
    }
    print!("(Type and press enter) ");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            break;
        }

        if let Some(item) = menu.get(input) {
            order.push(item.clone());
        } else {
            println!("{} is not on the menu.", input);
        }
    }

    customer.order = order;
}

/// This function is used to generate a random order for a customer.
/// It assigns the order to the customer.
/// The number of items in the order is random.
/// The items in the order are randomly selected from the menu.
pub fn get_random_order(customer: &mut Customer) {
    let menu = MENU_LIST.lock().unwrap();
    let mut order = Vec::new();

    for _ in 0..1 + rand::random::<usize>() % menu.len() {
        let item = menu
            .values()
            .nth(rand::random::<usize>() % menu.len())
            .unwrap();
        order.push(item.clone());
    }

    customer.order = order;
}

/// This function is used to extend the menu with a new item.
/// It takes the name and price of the item as arguments.
pub fn extend_menu(name: &'static str, price: f64) {
    let mut menu = MENU_LIST.lock().unwrap();
    menu.insert(name, MenuItem::new(name, price));
}
