use crate::draw::Draw;

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            components: Vec::new(),
        }
    }

    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
