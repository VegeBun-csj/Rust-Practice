// 实现一个打印图形面积的函数，接受一个可以计算面积的类型作为参数
// 比如圆形，三角形，正方形，需要使用泛型和泛型约束



fn main(){
    let circle = Circle{radius: 3.0};
    let triangle = Triangle{bottom: 2.0,height: 3.0};
    let square = Square { side: 2.0 };

    println!("圆的面积为:{}",compute(circle));
    println!("三角形的面积为:{}",compute(triangle));
    println!("正方形的面积为:{}",compute(square));

}


fn compute<T: ComputeArea>(graph: T) -> f32{
    graph.area()
}

struct Circle{
    radius : f32,
}

struct Triangle{
    bottom : f32,
    height : f32,
}

struct Square{
    side : f32,
}


pub trait ComputeArea {
    fn area(&self) -> f32;
}


impl ComputeArea for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * core::f32::consts::PI
    }
}


impl ComputeArea for Triangle {
    fn area(&self) -> f32 {
        self.bottom * self.height
    }
}


impl ComputeArea for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}