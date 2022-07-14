pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Light {
    // seconds
    fn duration(self) -> u32;
}

impl Light for TrafficLight {
    fn duration(self) -> u32 {
        match self {
          TrafficLight::Red => 3,
          TrafficLight::Green => 5,
          TrafficLight::Yellow => 7,
        }
    }
}

fn main() {
    let red_time = TrafficLight::Red.duration();
    let green_time = TrafficLight::Green.duration();
    let yellow_time = TrafficLight::Yellow.duration();
    println!("Red Light time: {red_time}");
    println!("Green Light time: {green_time}");
    println!("Yellow Light time: {yellow_time}");
}
