use std::collections::HashMap;

fn main() {
    let integers = get_integers(35);
    println!("{:?}", integers);
    println!("Mean: {:.2}", mean(&integers));
    println!("Mode: {}", mode(&integers));
    println!("Median: {}", median(&integers));
    println!("Standard Deviation: {:.2}", std(&integers));
}

fn get_integers(n: u32) -> Vec<i32> {
    let mut integers = Vec::new();
    for _i in 0..n {
        integers.push(rand::random::<i32>() % n as i32);
    }
    integers.sort();
    integers
}

fn mean(integers: &Vec<i32>) -> f64 {
    let sum: i32 = integers.iter().sum();
    sum as f64 / integers.len() as f64
}

fn median(integers: &Vec<i32>) -> f64 {
    let n = integers.len();
    if n % 2 == 0 {
        return (integers[n / 2] + integers[n / 2 - 1]) as f64 / 2.0;
    }
    integers[n / 2] as f64
}

fn mode(integers: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for i in integers {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;
    for (k, v) in counts {
        if v > max {
            max = v;
            mode = *k;
        }
    }

    mode
}

fn std(integers: &Vec<i32>) -> f64 {
    let mean: f64 = mean(integers);
    let sum: f64 = integers
        .iter()
        .map(|x| (x - mean as i32).pow(2) as f64)
        .sum();
    (sum / integers.len() as f64).sqrt()
}
