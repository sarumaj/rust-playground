use crate::draw::Draw;

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Button {
    pub fn new(width: u32, height: u32, label: &str) -> Button {
        Button {
            width,
            height,
            label: label.to_string(),
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button with width: {}, height: {}, and label: {}",
            self.width, self.height, self.label
        );
    }
}
