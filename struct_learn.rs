#[derive(Clone)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

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
}
