use std::option::Option;

// 一个简单的“方向”枚举，没有关联数据
enum Direction {
    North,
    South,
    East,
    West,
}
//带关联数据的枚举，每个变体都可以有不同的字段
enum Message {
    Quit,//单元变体
    Move{x:i32,y:i32,}, //结构体变体
    Write(String),//元组变体
    ChangeColor(i32,i32,i32),//元组变体
}

fn process(msg:Message){
    match msg {
        Message::Quit=>{
            println!("退出程序!!!");
        }
        Message::Move {x,y}=>{
            println!("移动到({},{})",x,y);
        }
        Message::ChangeColor(r, g, b)=>{
            println!("改变颜色：rgb({},{},{})",r,g,b);
        }
        Message::Write(text) => {
            println!("写入文字：{}", text);
        }
    }
}

fn main(){
    let quit =Message::Quit;
    process(quit);
    let m =Message::Move { x: 10, y: 20 };
    process(m);
    let changecolor =Message::ChangeColor(255, 255, 0);
    process(changecolor);
    let write=Message::Write(String::from("111111"));
    process(write);

}
