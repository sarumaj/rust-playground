use ::restaurant::*;

fn main() {
    let customers = vec![
        Customer::new("Alice"),
        Customer::new("Bob"),
        Customer::new("Carol"),
        Customer::new("Dave"),
        Customer::new("Eve"),
        Customer::new("Frank"),
        Customer::new("Grace"),
        Customer::new("Heidi"),
        Customer::new("Ivan"),
        Customer::new("Judy"),
        Customer::new("Kevin"),
        Customer::new("Lana"),
        Customer::new("Mallory"),
        Customer::new("Nia"),
        Customer::new("Oscar"),
        Customer::new("Peggy"),
        Customer::new("Quinn"),
        Customer::new("Rupert"),
        Customer::new("Sue"),
        Customer::new("Trent"),
        Customer::new("Ursula"),
        Customer::new("Victor"),
        Customer::new("Wendy"),
        Customer::new("Xander"),
        Customer::new("Yvonne"),
        Customer::new("Zelda"),
    ];

    extend_menu("Noodles", 16.39);
    extend_menu("Water", 0.89);

    for customer in customers {
        add_to_wait_list(customer);
    }

    add_table(1);

    let mut revenue = 0.0;
    loop {
        process_wait_list();

        take_orders(true);

        serve_orders();

        let daily_revenue = take_payments();
        if daily_revenue == 0.0 {
            break;
        }

        revenue += daily_revenue;
    }

    println!("Total revenue: ${:.2}.", revenue);
}
