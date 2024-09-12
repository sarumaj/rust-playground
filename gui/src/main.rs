use ::gui::{Button, Screen, SelectBox};

fn main() {
    let mut screen = Screen::new();
    screen
        .components
        .push(Box::new(SelectBox::new(75, 10, vec!["Yes", "No", "Maybe"])));
    screen.components.push(Box::new(Button::new(50, 10, "OK")));
    screen.run();
}
