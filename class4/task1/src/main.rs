enum Light {
    Red,
    Green,
    Yellow,
}

pub trait Time {
    fn time(&self) -> u8;
}

impl Time for Light {
    fn time(&self) -> u8 {
        match self {
            Light::Red => 30,
            Light::Green => 60,
            Light::Yellow => 10,
        }
    }
}

fn main() {
    let red_light = Light::Red;
    println!("red light: {}", red_light.time());

    let green_light = Light::Green;
    println!("green light: {}", green_light.time());

    let yellow_light = Light::Yellow;
    println!("yellow light: {}", yellow_light.time());
}
