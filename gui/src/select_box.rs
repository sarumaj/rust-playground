use crate::draw::Draw;

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl SelectBox {
    pub fn new(width: u32, height: u32, options: Vec<&str>) -> SelectBox {
        SelectBox {
            width,
            height,
            options: options.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a select box with width: {}, height: {}, and options: {:?}",
            self.width, self.height, self.options
        );
    }
}
