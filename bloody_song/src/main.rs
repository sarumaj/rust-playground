fn main() {
    for i in 1..=12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            numeral(i).unwrap()
        );

        for j in (1..=i).rev() {
            if j == 1 && i != 1 {
                print!("and ");
            }

            print!("{}", gift(j).unwrap());
            
            if j == 1 {
                println!(".\n");
            } else {
                println!(",");
            }
        }
    }
}

fn numeral(n: u32) -> Option<String> {
    return match n {
        1 => Some("first".to_string()),
        2 => Some("second".to_string()),
        3 => Some("third".to_string()),
        4 => Some("fourth".to_string()),
        5 => Some("fifth".to_string()),
        6 => Some("sixth".to_string()),
        7 => Some("seventh".to_string()),
        8 => Some("eighth".to_string()),
        9 => Some("ninth".to_string()),
        10 => Some("tenth".to_string()),
        11 => Some("eleventh".to_string()),
        12 => Some("twelfth".to_string()),
        _ => None,
    };
}

fn gift(n: u32) -> Option<String> {
    match n {
        1 => Some("a partridge in a pear tree".to_string()),
        2 => Some("two turtle doves".to_string()),
        3 => Some("three French hens".to_string()),
        4 => Some("four calling birds".to_string()),
        5 => Some("five golden rings".to_string()),
        6 => Some("six geese a-laying".to_string()),
        7 => Some("seven swans a-swimming".to_string()),
        8 => Some("eight maids a-milking".to_string()),
        9 => Some("nine ladies dancing".to_string()),
        10 => Some("ten lords a-leaping".to_string()),
        11 => Some("eleven pipers piping".to_string()),
        12 => Some("twelve drummers drumming".to_string()),
        _ => None,
    }
}
