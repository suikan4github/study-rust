fn main() {
    let x: [i32; 6] = [1, 3, 5, 7, 11, 13];
    println!("x[3] is {}", x[3]);

    println!("x.len is {}", x.len());

    for i in 0..x.len() {
        println!("x[{}] is {}", i, x[i]);
    }
}
