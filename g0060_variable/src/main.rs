fn main() {
    let foo = 3.14f64; // 変数の型は初期値から推論される。
                       //    foo = 2.17878; // immutableな変数には再代入できない。
    println!("foo is {}", foo);

    let bar: f32 = 1.41421356; // 変数宣言には型注釈できる。
    println!("bar is {}", bar);

    let mut baz = "The quick brown fox"; // mutableな変数には再代入できる。
    println!("baz is {}", baz);
    baz = "The rain in Spain";
    println!("baz is {}", baz);
}
