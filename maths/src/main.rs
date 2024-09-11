use ::maths::{get_integers, mean, median, mode, std};

fn main() {
    let integers = get_integers::<i32>(&35, 100);
    println!("{:?}", integers);
    println!("Mean: {:.2}", mean(&integers));
    println!("Mode: {}", mode(&integers));
    println!("Median: {}", median(&integers));
    println!("Standard Deviation: {:.2}", std(&integers));

    let integers = get_integers::<u32>(&35, 100);
    println!("{:?}", integers);
    println!("Mean: {:.2}", mean(&integers));
    println!("Mode: {}", mode(&integers));
    println!("Median: {}", median(&integers));
    println!("Standard Deviation: {:.2}", std(&integers));
}
