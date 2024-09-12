use ::avg::Avg;

fn main() {
    let mut avg = Avg::new();
    avg.add(1.5);
    avg.add(2.5);
    avg.add(3.5);
    avg.add(-4.5);
    avg.add(5.5);
    avg.remove();

    println!("Average: {}", avg.get());
}
