fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    // let x = tup.0;
    // let y = tup.1;
    // let z = tup.2;

    println!("Tuple: {x}, {y}, {z}");

    // arrays
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    let x = numbers[2];

    println!("numbers[2]: {x}")
}
