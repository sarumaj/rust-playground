use core::panic;

use super::{get_integers, mean, median, mode, std};

#[test]
fn test_get_integers() {
    let integers = get_integers::<i32>(&35, 100);
    assert_eq!(integers.len(), 100, "Length of integers is not 100");
    for i in integers {
        assert!(
            i >= -35 && i <= 35,
            "Integer {} is out of range [-35, 35]",
            i
        );
    }
}

#[test]
fn test_mean() {
    let integers = vec![1, 2, 3, 4, 5];
    let result = mean(&integers);
    assert_eq!(
        result, 3.0,
        "Mean of {:?} is not correct: {} != 3.0",
        integers, result
    );
}

#[test]
fn test_median() {
    let helper = |integers: Vec<i32>, expected: f64| {
        let result = median(&integers);
        assert_eq!(
            result, expected,
            "Median of {:?} is not correct: {} != {}",
            integers, result, expected
        );
    };

    helper(vec![1, 2, 3, 4, 5], 3.0);
    helper(vec![1, 2, 3, 4, 5, 6], 3.5);
}

#[test]
fn test_mode() {
    let integers = vec![1, 2, 3, 4, 5, 5];
    let result = mode(&integers);
    assert_eq!(
        result, 5,
        "Mode of {:?} is not correct: {} != 5",
        integers, result
    );
}

#[test]
fn test_std() {
    let integers = vec![1, 2, 3, 4, 5];
    let result = std(&integers);
    assert_eq!(
        result, 1.4142135623730951,
        "Standard deviation of {:?} is not correct: {} != 1.4142135623730951",
        integers, result
    );
}

#[test]
#[should_panic(expected = "Make this test fail")]
fn test_panic_example() {
    panic!("Make this test fail");
}
