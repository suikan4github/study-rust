# 列挙型の値に束縛された値
## この例題について

列挙型の値に束縛されたについて学習する。
- Ubuntu 24.04 (WSL)
- rustc 1.82.0
- cargo 1.82.0

プロジェクトはCargoで作った

## 例題実行手順
例題を実行するにはプロジェクトの中で以下のコマンドを実行する。
```sh
cargo run
```
## 内容
非常に奇妙な事だが、rustでは列挙型の値に別の型の値を束縛することができる。これは、C++のunionと同じ働きをする。union型を作らずに列挙型をこのような奇妙な形にした理由は不明である。



### 型定義
列挙型の定義において、列挙値の後ろに`()`で束縛される型名を書くと、列挙型の値に別の型の値を束縛できる。
```rust
enum Complex {
    Cartesian(CartesianComplex),
    Polar(PolarComplex),
}
```

この例では、列挙型ComplexがCartesianとPolarのふたつの値を持ち、それぞれCartesianComplex, PolarComplex型の値を束縛する。

### リテラル
値を束縛する列挙型のリテラルは、列挙型の値に続けて束縛される型の値を`()`で包んで記述する。

```rust
    let z = Complex::Cartesian(CartesianComplex {
        real: 1.0,
        imaginary: 2.0,
    });
```
この例ではCartesianComplexは構造体型である。

### match文
単純な列挙型と同じく、match文による処理が可能である。この場合、アームの左辺の値は列挙型の値であるが、`()`に包んで仮引数も指定する。
```
match文 ::= "match" 式 "{" アームの列 "}"
アームの列 ::= アーム { アームの列 }
アーム ::= 値 "=>" 式 ","
```
仮引数には、実行時にその列挙型に束縛された値が渡される。この仮引数は式の中で利用できる。
```rust
fn print_complex_in_polar(complex: &Complex) {
    match complex {
        Complex::Cartesian(cartesian) => {
            let polar = cartesian.to_polar();
            println!("Polar: radius = {}, angle = {}", polar.radius, polar.angle);
        }
        Complex::Polar(polar) => {
            // 極座標はそのまま表示する
            println!("Polar: radius = {}, angle = {}", polar.radius, polar.angle);
        }
    }
}
```

