#[cfg(test)]
mod tests;

use num::Integer;
use rand::{distributions::Standard, prelude::Distribution};
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Sum;

pub fn get_integers<N>(max: &N, size: i32) -> Vec<N>
where
    N: num::Integer + Clone,
    Standard: Distribution<N>,
{
    let mut integers = Vec::new();

    for _ in 0..size {
        // Generate a random value of type N
        let mut random_value: N = rand::random::<N>();
        random_value = random_value.rem(max.clone());

        // Use modulo to fit within the range
        integers.push(random_value);
    }

    integers.sort();
    integers
}

pub fn mean<N>(integers: &Vec<N>) -> f64
where
    N: Integer + Clone + Into<f64> + Sum,
{
    let sum = integers.iter().map(|x| x.clone()).sum::<N>();
    sum.into() / integers.len() as f64
}

pub fn median<N>(integers: &Vec<N>) -> f64
where
    N: Integer + Clone + Copy + Into<f64>,
{
    let n = integers.len();
    if n % 2 == 0 {
        return (integers[n / 2] + integers[n / 2 - 1]).into() / 2.0;
    }
    integers[n / 2].into()
}

pub fn mode<N>(integers: &Vec<N>) -> N
where
    N: Integer + Clone + Copy + From<bool> + Hash,
{
    let mut counts = HashMap::new();
    for i in integers {
        let count = counts.entry(i).or_insert(N::from(false));
        count.inc();
    }

    let mut max: N = N::from(false);
    let mut mode: N = N::from(false);
    for (k, v) in counts {
        if v > max {
            max = v;
            mode = *k;
        }
    }

    mode
}

pub fn std<N>(integers: &Vec<N>) -> f64
where
    N: Integer + Clone + Into<f64> + Sum,
{
    let mean: f64 = mean(integers);
    let sum: f64 = integers
        .iter()
        .map(|x| x.clone().into())
        .map(|x| (x - mean).powi(2))
        .sum();
    (sum / integers.len() as f64).sqrt()
}
