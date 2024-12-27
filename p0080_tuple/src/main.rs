fn main() {
    // Declaration of tuple
    let _a: (u32, f64, &str) = (24, 3.141592, "Hello, world");

    // Element access
    println!("_a.0 is {}", _a.0);

    // Destructuring assignment
    let (_x, _y, _z) = _a;

    println!("_x is {}", _x);
    println!("_y is {}", _y);
    println!("_z is {}", _z);
}
