//枚举的学习 
enum IpaddrKind {
    v4(u8,u8,u8,u8),
    v6(String),
}
struct IpAddr {
    kind:IpaddrKind,
    address:String
}

fn main(){
    let home=IpaddrKind::v4(12, 16, 0, 0);
    let local=IpaddrKind::v6("111");
    let first= IpAddr{
        kind:IpaddrKind::v4,
        address:String::from("1352744183@qq.com")
    };
    let second=IpAddr{
        kind:IpaddrKind::v6,
        address:"fgg@163.com".to_string()
    };

}
