use std::f64::consts;
struct Rectangle {
    a: f64,
    b: f64,
}

struct Circle {
    r: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

// 定义接口
pub trait Caculate{
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
}

// 定义一个enum
enum StructType {
    Rectangle(f64,f64),
    Circle(f64),
    Triangle(f64,f64,f64),
}


pub struct Pic{
    struct_type: StructType,
}

impl Pic{
    fn new(struct_type: StructType,) -> Pic {
        Pic{struct_type}
    }
}


// 为pic结构体实现Caculate接口
impl Caculate for Pic{
    fn perimeter(&self) -> f64{
        match self.struct_type{
            StructType::Rectangle(a,b) => (a+b) * 2.0,
            StructType::Circle(a) => 2.0 * consts::PI * a,
            StructType::Triangle(a,b,c) => a + b + c,
        }
    }

    fn area(&self) -> f64 {
        match self.struct_type{
            StructType::Rectangle(a,b) => a * b,
            StructType::Circle(a) => a * a * consts::PI,
            StructType::Triangle(a,b,c) => {
                let p = (a + b + c) / 2.0;
                (p * (p - a) * (p - b) * (p - c)).sqrt()
            },
        }
    }
}



fn main() {
    let pic = Pic::new(StructType::Rectangle(2.0,3.0));
    let area = pic.area();
    println!("area is {}",area);
}
