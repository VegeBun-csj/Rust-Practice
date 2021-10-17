///
/// >>>>>>>>>>>>>>>>>>>>>>>>>>>>爬取网页数据，并将html数据格式转换为markdown<<<<<<<<<<<<<<<<<<<<<<<<<<<<
/// 
/* use std::fs;
fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("fetching url is : {}", url);

    //这里使用unwrap()只会关心正确结果，如果想要错误进行传播，可以使用？
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}
 */


///
/// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>函数返回值有分号就是返回uint<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
/// 
/* fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}

fn main() {
    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };

    println!(
        "is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}",
        is_pi, is_unit1, is_unit2
    );
} */


///
/// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 聊天室 <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
/// 
/* #[derive(Debug)]
enum Gender{
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

// 元组结构体
#[derive(Debug, Clone, Copy)]
struct UserId(u64);

#[derive(Debug, Clone, Copy)]
struct TopicId(u64);

// 标准结构体
#[derive(Debug)]
struct User{
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic{
    id: TopicId,
    name: String,
    owner: UserId,
}

// 定义聊天室中可能发生的事件
#[derive(Debug)]
enum Event{
    Join((UserId,TopicId)),
    Leave((UserId,TopicId)),
    Message((UserId,TopicId,String)),
}

fn main(){
    let alice = User{id: UserId(1),name: "Alice".into(), gender: Gender::Female};
    let bob = User{id: UserId(2),name: "Bob".into(), gender: Gender::Male};
    let topic = Topic{id: TopicId(1), name: "rust".into(), owner: UserId(1)};

    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Leave((alice.id, topic.id));
    let event4 = Event::Message((bob.id, topic.id, "hello world".into()));

    println!("event1 is {:?}, event2 is {:?}, event3 is {:?}, event4 is {:?}", event1, event2, event3, event4);
} */


///
/// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>流程控制<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
/// (通过非波那且数列进行演示)
/* 
fn fib_loop(n: u8){
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop{
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);                
        if i >= n {            
            break;        
        }
    }
}


fn fib_while(n: u8){
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n {
        let c = a + b;        
        a = b;        
        b = c;        
        i += 1;                
        println!("next val is {}", b);
    }
}

fn fib_for(n: u8){
    let (mut a, mut b) = (1, 1);        
    for _i in 2..n {        
        let c = a + b;        
        a = b;        
        b = c;        
        println!("next val is {}", b);    
    }
}


fn main() {    
    let n = 10;    
    fib_loop(n);    
    fib_while(n);    
    fib_for(n);
} */




