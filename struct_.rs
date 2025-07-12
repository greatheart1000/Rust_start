//struct 结构体
struct User {
    email:String,
    name:String,
    active:bool,
    sign_in_out:u64
}

fn main(){
   println("hello world");
   let mut user1 =User(
    email:String::from("135qq.com"),
    name:String::from("cj"),
    active:true.
    sign_in_out:1
   );
   user2 =User(
    email:String::from("111@qq.com"),
    name:String::from("vbn"),
    active:user1.active,
    sign_in_out:user1.sign_in_out
   )

}
//struct tuple
strcut color(u32,u32,u32);
struct Point(u32,u32,u32);
let a1 =color(0,0,0);
let b1=Point(255,255,255);


enum Message{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),}
impl Message {
    fn call(&self){}
}
fn main(){
    let q = Message::Quit;
    let m = Message::Move {x: 12,y: 24 };
    let W = Message::Write(String:: from("Hello"));
    let C = Message::ChangeColor(0, 255, 255);
m.call();
}
