fn main() {
    let mut wall = Wall::new();

    loop {
        let mut input = String::new();
        println!("Enter a brick (width height depth) or \"quit\" to quit:");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "quit" {
            break;
        }

        let mut parts = input.trim().split_whitespace();
        let width: u32 = parts
            .next()
            .expect("No width entered")
            .parse()
            .expect("Width must be a number");
        let height: u32 = parts
            .next()
            .expect("No height entered")
            .parse()
            .expect("Height must be a number");
        let depth: u32 = parts
            .next()
            .expect("No depth entered")
            .parse()
            .expect("Depth must be a number");

        let mut brick = Brick::new(0, 0, 0);
        brick.resize(width, height, depth);

        wall.add_brick(brick);
    }

    println!("Surface area: {}", wall.surface_area());
    println!("Wall area: {}", wall.wall_area());
    println!("Volume: {}", wall.volume());
}

struct Brick {
    width: u32,
    height: u32,
    depth: u32,
}

impl Brick {
    fn new(width: u32, height: u32, depth: u32) -> Brick {
        Brick {
            width,
            height,
            depth,
        }
    }

    fn can_stack_on(&self, other: &Brick) -> bool {
        self.width == other.width && self.depth == other.depth
    }

    fn surface_area(&self) -> u32 {
        2 * (self.width * self.height + self.width * self.depth + self.height * self.depth)
    }

    fn resize(&mut self, width: u32, height: u32, depth: u32) {
        self.width = width;
        self.height = height;
        self.depth = depth;
    }

    fn wall_area(&self) -> u32 {
        self.width * self.height
    }

    fn volume(&self) -> u32 {
        self.width * self.height * self.depth
    }
}

struct Wall {
    bricks: Vec<Brick>,
}

impl Wall {
    fn new() -> Wall {
        Wall { bricks: Vec::new() }
    }

    fn add_brick(&mut self, brick: Brick) {
        if let Some(last_brick) = self.bricks.last() {
            if !brick.can_stack_on(last_brick) {
                println!("Brick does not fit on top of the last brick");
                return;
            }
            if brick.volume() > last_brick.volume() {
                println!("Brick is too large");
                return;
            }
        }
        self.bricks.push(brick);
    }

    fn surface_area(&self) -> u32 {
        self.bricks.iter().map(|brick| brick.surface_area()).sum()
    }

    fn wall_area(&self) -> u32 {
        self.bricks.iter().map(|brick| brick.wall_area()).sum()
    }

    fn volume(&self) -> u32 {
        self.bricks.iter().map(|brick| brick.volume()).sum()
    }
}
