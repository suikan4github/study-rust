// この例題は

#[derive(Debug, Clone)]
// 2次元の複素数を表す構造体
struct CartesianComplex {
    real: f64,
    imaginary: f64,
}

#[derive(Debug, Clone)]
// 極座標複素数を表す構造体
struct PolarComplex {
    radius: f64,
    angle: f64,
}

trait ConvertFromCartesian {
    fn to_polar(&self) -> PolarComplex;
}

// 直交座標を極座標に変換するトレイトの実装
impl ConvertFromCartesian for CartesianComplex {
    fn to_polar(&self) -> PolarComplex {
        let radius = (self.real.powi(2) + self.imaginary.powi(2)).sqrt();
        let angle = self.imaginary.atan2(self.real);
        PolarComplex { radius, angle }
    }
}

trait ConvertFromPolar {
    fn to_cartesian(&self) -> CartesianComplex;
}
// 極座標を直交座標に変換するトレイトの実装
impl ConvertFromPolar for PolarComplex {
    fn to_cartesian(&self) -> CartesianComplex {
        let real = self.radius * self.angle.cos();
        let imaginary = self.radius * self.angle.sin();
        CartesianComplex { real, imaginary }
    }
}

#[derive(Debug, Clone)]
// 複素数を表す列挙型
enum Complex {
    Cartesian(CartesianComplex),
    Polar(PolarComplex),
}

// 複素数を極座標で表示する関数
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

fn main() {
    let z = Complex::Cartesian(CartesianComplex {
        real: 1.0,
        imaginary: 2.0,
    });

    print_complex_in_polar(&z);

    let z = Complex::Polar(PolarComplex {
        radius: 1.0,
        angle: 2.0,
    });

    print_complex_in_polar(&z);
}
