fn main() {
    enum Style {
        Height(u32),
        Weight(u32),
    }


    let u= Style::Height(170);

    println!("{}", u::Height);

}
