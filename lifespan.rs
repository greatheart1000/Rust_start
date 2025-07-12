//生命周期
生命周期标注语法
·生命周期的标注不会改变引用的生命周期长度
当指定了泛型生命周期参数，函数可以接收带有任何生命周期的引用
·生命周期的标注：描述了多个引用的生命周期阅的关系，但不影响生命周期
fn main(){
    let r;
    {
        let x=5;
         r =&x;
    }
    println!("{}",r);
}
// 报错`x` does not live long enough 下面这样写不会报错
fn main(){
    let z=5;
    let r = &z;
    println!("{}",r);
}
