# ヒープ上に確保した変数の廃棄
## この例題について
ある種の型はヒープ上に確保される。Rustでは、ヒープ上に確保された変数は、スコープを抜けると自動的に廃棄される。
- Ubuntu 24.04 (WSL)
- rustc 1.86.0
- cargo 1.86.0

プロジェクトはCargoで作った。

## 例題実行手順
例題を実行するにはプロジェクトの中で以下のコマンドを実行する。
```sh
cargo run
```
## 内容
文字列リテラルは静的に確保され、その生存期間はプログラムの実行期間と同じである。文字列リテラルを値としてもつ変数の型は`&'static str`である。文字列リテラルはヒープ上に確保されない。また、性質上固定長である。

```rust
    let str_message: &'static str = "Hello, static str!";
```

ここで`&'static str`は、静的な文字列スライスを表す型である。`&`は参照を表し、`'static`はその参照のライフタイムがプログラム全体にわたることを示す。`'static`のような記述はライフタイム注釈と呼ばれ、Rustの所有権システムにおいて重要な役割を果たす。

ヒープ上に確保される文字列は、`String`型である。`String`型は、可変長の文字列を表現するために使用される。ヒープ上に確保された変数は、スコープを抜けると自動的に廃棄される。

```rust
    let mut string_message: String = String::from("Hello, String!");
```

厳密に言えば、スコープを抜けると自動的に廃棄されるのは、`String`型の所有権を持つ変数である。このとき変数に束縛されている`String`型の値も同時に廃棄される。

そしてこの変数に束縛されていない`String`型の値に対して変数は所有権を持っていないため、そのような値が同時に廃棄されることはない。

```rust
    // string_messageは値が"Hello, String!"であるString値の所有権を持つ。
    let mut string_message: String = String::from("Hello, String!");

    // string_messageは値が"Hello, String2!"であるString値の所有権を持つ。
    // ここでstring_messageは新しい値に置き換えられ、古い値は廃棄される。
    string_message = "Hello, String2!".to_string();
    // string_messageのスコープを出ると、その値である"Hello, String2!"は自動的に廃棄される。
```

