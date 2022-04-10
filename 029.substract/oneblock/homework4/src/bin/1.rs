/// 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
fn main() {
    let (red_light, 
        yellow_light, 
        green_light) =
        (   TrafficLight::Red, 
            TrafficLight::Yellow, 
            TrafficLight::Green);
    println!(
        "red light is: {}\n
        yellow light is: {}\n
        green light is: {}",
        red_light.time(),
        yellow_light.time(),
        green_light.time()
    );
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Light {
    fn time(&self) -> u8;
}

impl Light for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            Self::Red => 60,
            Self::Yellow => 5,
            Self::Green => 30,
        }
    }
}
