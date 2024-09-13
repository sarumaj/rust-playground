use ::zoo::{Animal, Cage, Cat, Dog};

fn main() {
    let cat = Cat;
    let dog = Dog;
    println!("Cat says: {}", cat.noise());
    println!("Dog says: {}", dog.noise());

    let cage1 = Cage::new(&cat);
    println!("Who is?: {}", cage1);
    println!("What it says?: {}", cage1.noise());

    let cage2 = Cage::new(&dog);
    println!("Who is?: {}", cage2);
    println!("What it says?: {}", cage2.noise());

    println!("Pet the cat: {}", Cat::pet());
    println!("Pet the dog: {}", Dog::pet());
    println!("Pet the cat in the cage: {}", Cage::<Cat>::pet());
    println!("Pet the dog in the cage: {}", Cage::<Dog>::pet());

    let freed = cage1.release();
    println!("Freed animal: {}", freed);

    let freed = cage2.release();
    println!("Freed animal: {}", freed);
}
