fn main() {
    struct Complex {
        re: f32,
        im: f32,
    }

    let u: Complex = Complex { re: 0.0, im: 1.0 };

    println!("{},{}", u.re, u.im);

    type ComplexArray = [Complex; 2];

    let a: ComplexArray = [Complex { re: 0.0, im: 1.0 }, Complex { re: 2.0, im: 3.0 }];

    println!("{},{}", a[0].re, a[0].im);
    println!("{},{}", a[1].re, a[1].im);
}
