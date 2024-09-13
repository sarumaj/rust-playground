use ::lin_alg::Point;

fn main() {
    let mut p = Point::new(1.0, 2.0);
    p += Point::new(3.0, 4.0);
    p.normalize();
    p *= 2.0;
    println!("Got: {}", p);

    // using destructuring (requires public fields)
    let Point(x, ..) = p; // ignore y
    println!("x: {}, y: {}", x, p.y());

    let other = Point::new(5.0, 6.0);
    println!("Distance: {}", p.distance(other));
}
