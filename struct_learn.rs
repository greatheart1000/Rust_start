#[derive(Clone)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//元组结构体
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

//没有任何字段的类单元结构体 下面是一个声明和实例化一个名为 AlwaysEqual 的 unit 结构的示例
struct AnyWal;

fn main(){
    let mut user1= User {
        active:true,
        username: "caijian".to_string(),
    email: "1362744183@qq.com".to_string(),
    sign_in_count:1,
    };
    user1.email="1372744183@qq.com".to_string();
    println!("{}",user1.email);
    let user2 =User{
        email:"133@qq.com".to_string(),
        ..user1.clone()
    };
    println!("{}",user2.email);
    let black=Color(0,0,0);
    let point =Point(0,0,0);
    let subject=AnyWal;
}

fn get_area(dimension:(u32,u32))->u32 {
    dimension.0*dimension.1
}

fn main(){
    let are=get_area((30,50));
    println!("{are}");
}
