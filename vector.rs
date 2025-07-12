fn main(){
    let mut v:Vec<u32> =Vec::new(); //创建一个vector 
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    let fitsrt =&v[0];
    println!("the num is {}",fitsrt);
    let Cv=vec![1,2,3,4,5,6];
    println!("the ans is {}",Cv[2]);
    //遍历vector里面的值
    let mut new_v= vec![1,2,3,4,5,6];
    for i in &mut new_v{
        *i +=50;}
    for i in new_v{
        println!("the ele is {}",i)
    }
    }

//enum 和 vec 

enum Sprin {
    Int(i32),
    Text(String),
    Float(f64),
    
}

fn main(){
    let row = vec![
        Sprin::Text(String::from("haha")),
        Sprin::Int(3),
        Sprin::Float(10.12),
    ];

}

创建一个新的字符串(String)
·很多Vec<T>的操作都可用于 String。
·String::new(） 函数
(例子)
·使用初始值来创建String:
－to_string(）方法，可用于实现了Display trait 的类型，包括字符串字面值（例子）
－ String:from(）函数，从字面值创建 String（例子)

更新 String
push_str(）方法：把一个字符串切片附加到 String（例子）
pushO方法：把单个字符附加到String(例子)
+：连接字符串（例子)
－使用了类似这个签名的方法fn add(self,s:&str)->String{..}
·标准库中的add方法使用了泛型
·只能把 &str 添加到 String
解引用强制转换（deref coercion）
fn main(){
    let mut a = String::from("haha");
    a.push_str("haha");
    println!("the string is {}",a);
    let b=String::from("hbhb");
    let c =a+&b;
    println!("the string is {}",c);
    //println!("the a is {}",a); //现在不能继续引用了 
    //println!("the b is {}",b);
}
           

           
