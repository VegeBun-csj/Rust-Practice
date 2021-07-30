// // 定义一个交通信号灯（枚举）
enum TrafficSignal {
    Red,
    Green,
    Yellow,
}

// 定义一个信号灯时间的trait
trait Signal {
    fn signal_time(&self) -> i32;
}

// 为TrafficSignal枚举实现这个trait
impl Signal for TrafficSignal {
    fn signal_time(&self) -> i32 {
        // 在其中根据不同的交通信号等显示不同的时间
        match &self {
            TrafficSignal::Red => 60,
            TrafficSignal::Green => 30,
            TrafficSignal::Yellow => 3,
        }
    }
}

fn main() {
    let red_light = TrafficSignal::Red;
    let green_light = TrafficSignal::Green;
    let yellow_light = TrafficSignal::Yellow;
    //
    println!("红灯的时间为{}", red_light.signal_time());
    println!("绿灯的时间为{}", green_light.signal_time());
    println!("黄灯的时间为{}", yellow_light.signal_time());
}
