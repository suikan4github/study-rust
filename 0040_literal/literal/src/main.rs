fn main() {
    println!("{}", 1000); // デフォルトの整数型
    println!("{}", 1032i32); // 符号付き32bit整数型を指定
    println!("{}", 1032u32); // 符号なし32bit整数型を指定

    //    println!("{}", -1032u32);  // 値の範囲を逸脱している

    println!("{}", 1016i16); // 符号付き16bit整数型

    //    println!("{}", 1023u8);

    println!("{}", 1064i64); // 符号付き64bit整数型
    println!("{}", 1128i128); // 符号付き128bit整数型

    //    println!("{}", 1256i256); // 256bit整数型はない

    println!("{}", std::any::type_name_of_val(&1000)); // デフォルト型名を表示

    println!("{}", 1_000); // 桁区切りを使える
    println!("{}", 0x1234_5678); // 16進数
    println!("{}", 0o123); // 8進数
    println!("{}", b'C'); // バイト定数
    println!("{}", std::any::type_name_of_val(&b'C')); // バイトリテラルの型名を表示

    println!("{}", 3.1415); // デフォルトの浮動小数点数型
    println!("{}", 6.022e23); // 指数表記

    println!("{}", 1.41421356f32); // IEEE32bit浮動小数点数を指定
    println!("{}", 1.41421356f64); // IEEE64bit浮動小数点数を指定
    println!("{}", std::any::type_name_of_val(&1.0)); // デフォルト型名を表示
}
