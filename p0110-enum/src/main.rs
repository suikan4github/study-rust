enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Caution!"),
        TrafficLight::Green => println!("Go!"),
    }
}

fn main() {
    let light = TrafficLight::Red;
    action(light);

    let light = TrafficLight::Green;
    action(light);
}
