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
