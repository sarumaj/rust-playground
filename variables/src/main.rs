fn main() {
    let _spaces = "   ";
    // Shadowing allows mutation of type
    let _spaces = _spaces.len();

    let mut _spaces = "    ";
    // _spaces = _spaces.len();
    // will throws mismatched types

    let mut y = 5;

    // Changing mutable y
    y = y + 1;

    {
        // Shadowing y
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
        // will be 12
    }

    println!("The value of x is: {y}");
    // will be 6

    let x = 5;

    // Shadowing x
    let x = x + 1;

    {
        // Shadowing x
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        // will be 12
    }

    println!("The value of x is: {x}");
    // will be 6
}
