enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(i32, i32, i32),
}

impl Message {
    fn print(self: &Self) {
        match self {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move: {}, {}", x, y),
            Message::Write(s) => println!("write: {}", s),
            Message::Color(r, g, b) => println!("color: {}, {}, {}", r, g, b),
        }
    }
}

fn main() {
    let m = Message::Quit;
    m.print();

    let m = Message::Move { x: 2, y: 3 };
    m.print();

    let m = Message::Write(String::from("hello world"));
    m.print();

    let m = Message::Color(120, 120, 120);
    m.print();

    let i = Option::Some(5);
    let r = 1 + match i {
        Some(x) => x,
        _ => 0,
    };

    println!("r = {}", r);

    let r2 = 1 + if let Some(x) = i { x } else { 0 };

    println!("r2 = {}", r2);
}
