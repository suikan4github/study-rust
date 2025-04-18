// この例題は

enum Flag {
    Yellow,
    RedCross,
    Red,
    Checker,
}

fn action(signal: Flag) {
    match signal {
        Flag::Yellow => println!("Do not over take!"),
        Flag::Red => println!("Return to pit!"),
        Flag::RedCross => println!("Rain race!"),
        Flag::Checker => println!("Finish!"),
    }
}

fn main() {
    let flag = Flag::Red;
    action(flag);

    let flag = Flag::Checker;
    action(flag);
}
