fn main() {
    // 文字リテラル
    println!("{}", 'a'); // 英数文字
    println!("{}", 'あ'); // Unicode文字

    //    println!("{}", 'ab'); // 文字リテラルは1文字だけが許される。

    // 文字裂リテラル
    println!("{}", "Alpha numeric string");
    println!("{}", "UNICODE文字列");

    // リテラルの型
    println!("{}", std::any::type_name_of_val(&'a')); // 文字テラルの型名を表示
    println!("{}", std::any::type_name_of_val(&"いろは")); // 文字列テラルの型名を表示

    // エスケープシーケンス
    println!("{}", "\"a string\""); // "\"によるエスケープ
                                    // Raw string
    println!("{}", r#""a string""#); // "..."の前後に0個以上の'#'を置き、その前に'r'を置くとrawリテラルになる
    println!("{}", r##""a#string""##); // 文字列の中にある#の連続数より多くの#で囲む。
    println!("{}", r###""a##string""###);
}
