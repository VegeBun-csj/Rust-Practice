use rand::Rng;
use std::cmp::Ordering;
use std :: io;


fn main() {
    println!("Hello, world!");
    guess();
}




//猜数游戏
fn guess(){
    println!("猜数游戏");
    let secrect_num = rand::thread_rng().gen_range(1..101);

    loop{
        println!("猜测一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32= match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数字是: {}",guess);
        match guess.cmp(&secrect_num){
            Ordering::Less => println!("too small"),    //注意这里是逗号
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you Win !");
                break;
            }
        }
    }
}
