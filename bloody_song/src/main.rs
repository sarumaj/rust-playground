fn main() {
    for i in 1..=12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            cardinal(i).unwrap_or("unknown month")
        );

        for j in (1..=i).rev() {
            if j == 1 && i != 1 {
                print!("and ");
            }

            print!("{}", gift(j).unwrap_or("unknown gift"));

            if j == 1 {
                println!(".\n");
            } else {
                println!(",");
            }
        }
    }
}

fn cardinal(n: u32) -> Option<&'static str> {
    return match n {
        1 => Some("first"),
        2 => Some("second"),
        3 => Some("third"),
        4 => Some("fourth"),
        5 => Some("fifth"),
        6 => Some("sixth"),
        7 => Some("seventh"),
        8 => Some("eighth"),
        9 => Some("ninth"),
        10 => Some("tenth"),
        11 => Some("eleventh"),
        12 => Some("twelfth"),
        _ => None,
    };
}

fn gift(n: u32) -> Option<&'static str> {
    return match n {
        1 => Some("a partridge in a pear tree"),
        2 => Some("two turtle doves"),
        3 => Some("three French hens"),
        4 => Some("four calling birds"),
        5 => Some("five golden rings"),
        6 => Some("six geese a-laying"),
        7 => Some("seven swans a-swimming"),
        8 => Some("eight maids a-milking"),
        9 => Some("nine ladies dancing"),
        10 => Some("ten lords a-leaping"),
        11 => Some("eleven pipers piping"),
        12 => Some("twelve drummers drumming"),
        _ => None,
    };
}
