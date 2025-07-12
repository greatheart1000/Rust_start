Result 枚举
enum Result<T, E>{
Ok(T),
Err(E),
）T：操作成功情况下，k变体里返回的数据的类型
E：操作失败情况下，Err变体里返回的错误的类型

use std::fs::File;
fn main(){
    //panic!("carsh ");
    let fs = File::open("hello.txt");
    let f = match fs {
        Ok(file)=>file,
        Err(error)=>panic!("cannot open the file")  
    };
} 
//panic宏 可恢复错误与不可恢复错误
fn main(){
    //panic!("carsh ");
    let fs = File::open("hello.txt").expect("无法打开文件");
}

use std::fs::File;
use std::io;
use std::io ::Read;
fn read_username_from_file()->Result<String,io::Error>{
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) }
fn main(){
    let result = read_username_from_file();
}
// ?在处理错误的作用 
use std::error::Error; 
use std::fs::File;
fn main()->Result<(),Box<dyn Error>>{
    let s:File =File::open("hello.txt")?;
     Ok(())
}
编写示例、原型代码、测试
可以使用 panic!
－演示某些概念：：unwrap
－原型代码：unwrap、expect
－测试：unwrap、expect

错误处理的指导性建议
·当代码最终可能处于损坏状态时，最好使用 panic!
·损坏状态（Bad state）：某些假设、保证、约定或不可变性被打破
－例如非法的值、矛盾的值或空缺的值被传入代码
－以及下列中的一条：
·这种损坏状态并不是预期能够偶尔发生的事情。
·在此之后，您的代码如果处于这种损坏状态就无法运行。
·在您使用的类型中没有一个好的方法来将这些信息（处于损坏状态）进行编码。

use std::net::IpAddr;
fn main(){
    let home:IpAddr="127.0.0.1".parse().unwrap();
}

fn main(){
loop {
    let guess = "32";
    let guess: i32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) =>continue,};

if guess < 1||guess> 100 {
    println!("The secret number will be between 1 and 100.");
    continue;}
}
}
//创建结构体以及关联函数 
pub struct Guess{
    value:i32,
}
impl Guess {
    pub fn new(value:i32)->Guess{
        if (value<1 ||value>100){
            panic!("guess value must 1 and 100,{}",value);
        }
        Guess { value: () }
    }
    pub fn value(&self)->i32{
        self.value
    }
}
fn main(){
loop {
    let guess = "32";
    let guess: i32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) =>continue,};

if guess < 1||guess> 100 {
    println!("The secret number will be between 1 and 100.");
    continue;}
}
}
