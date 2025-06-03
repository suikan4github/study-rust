fn main() {
    // 静的な文字列リテラル
    let str_message: &'static str = "Hello, static str!";
    // ヒープに格納される可変文字列
    let mut string_message: String = String::from("Hello, String!");

    println!("Static str: {}", str_message);
    println!("String: {}", string_message);

    // String型の変数の束縛を変更すると、直前に束縛されていた値
    // (ヒープに格納されている) は自動的にドロップされる。
    // プログラマはこの動作を意識する必要はない。
    string_message = "Hello, String2!".to_string();
    println!("Overridden String: {}", string_message);

    // string_messageに束縛されている値は、ヒープに格納されている。
    // ヒープに格納されている値は、スコープを抜けると自動的にドロップされる。
    // そのため、明示的にドロップする必要はない。
}
