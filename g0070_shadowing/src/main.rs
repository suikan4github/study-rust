fn main() {
    let foo = "Hello";
    println!("foo is {}", foo);
    // foo = "World"; // not allowed.
    let foo = "World"; // shadowing by redeclaration. Allowed.
    println!("foo is {}", foo);

    {
        let foo = "from Rust"; // shadowing from nested scope. Allowed.
        println!("foo is {}", foo);
    }

    println!("foo is {}", foo); // The variable in the nested scope is gone.

    let foo = foo.len();
    println!("foo is {}", foo);
}
