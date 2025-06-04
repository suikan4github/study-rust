fn main() {
    //    let mut stack: Vec<&'static str> = Vec::new();
    let mut stack = Vec::new();

    stack.push("the lazy dog.");
    stack.push("jumps over ");
    stack.push("The quick brown fox");

    println!("Stack contents: {:?}", stack.pop());
    println!("Stack contents: {:?}", stack.pop());
    println!("Stack contents: {:?}", stack.pop());
}
