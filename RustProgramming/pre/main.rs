/* use std::f64::consts;
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
 */

fn main() {
    // let mut v = vec![1, 2, 3];
    // assert_eq!(v.remove(1), 2);
    // assert_eq!(v, [1, 3]);
    // remove的是下标,时间复杂度为O(n)
    // v.remove(2);
    // println!("{:?}",v);
    // println!("{}",v.get(2))

    // for (i, el) in v.iter().enumerate() {
    //     println!("The current element is {}", el);
    //     println!("The current index is {}", i);
    // }

    // let v = vec!["a".to_string(), "b".to_string()];
    // for s in v.into_iter() {
    //     // s has type String, not &String
    //     println!("{}", s);
    // }
    // v.into_iter()
    // .map(|i|{
    //     println!("{}", i)
    // });

    let mut v = vec![1,2,3,4,5,6];
    // let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];

    // v.sort_by(|a, b| b.cmp(a));
    // .into_iter()
    // .map(|a| a + 1)
    // .collect();

    // v.reverse();
    // for i in 0..v.len(){
    //     if let Some(val) = v.get(i){
    //         println!("{}",val);
    //     }
    // }
    // println!("{:?}", v);


    /*     let mut a = 30;

        let mut counter = 30;
        while counter >= 0 {
            counter -= 1;
            let mut id = a - 1;
            println!("a为{:?}", a);
            println!("计数器为:{:?}", counter);
        } */
    
    
}
