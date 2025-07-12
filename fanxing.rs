//去重 泛型 

fn largest(list:&[i32])->i32{
    let mut largest_num =list[0];
    for &item in list{
        if (item>largest_num){
            largest_num =item
        }
    }
    largest_num
}

fn main(){
    let list1 =vec![1,2,3,4,5,6];
    let num =largest(&list1);
    println!("the num is {}",num);
    let list2 =vec![2,3,4,5,6,7,8];
    let numq=largest(&list2);
    println!("the num is {}",numq);

}

struct Point<T>{
    x:T,
    y:T
}
fn main(){
    let intergate =Point{x:10,y:10};
    let bbb =Point{x:255,y:100};  
}
//不同的类型 结构体中的泛型 
struct Point<T,U>{
    x:T,
    y:U
}
fn main(){
    let intergate =Point{x:10,y:10.1};
    let bbb =Point{x:255,y:100.5};  
}

//枚举中的泛型 
enum Option<T>{
    Some(T),
    None
}

enum Result<T,E>{
    Ok(T),
    Err(E)
}
fn main(){   
}

//方法中的泛型
struct Point<T>{
    x:T,
    y:T,
}
enum Option<T>{
    Some(T),
    None
}

enum Result<T,E>{
    Ok(T),
    Err(E)
}

fn main(){
    let P =Point{
        x:5,y:10};
        println!("num is {} ",P.x);
}


struct Point<T> {
    x: T,
    y: T,
}

在 Rust 里，你看到的

```rust
struct Point<T> {
    x: T,
    y: T,
}

// 为所有 T 实例化 Point<T> 添加方法
impl<T> Point<T> {
    // 一个“实例方法”（method），不是关联函数（associated function）
    // 它的签名等价于 fn x(self: &Point<T>) -> &T
    fn x(&self) -> &T {
        &self.x
    }
}
```

拆开来解释一下：

1. `impl<T> Point<T>`  
   - `impl` 块后面跟的 `<T>`，表示「对任意类型 `T`」都给 `Point<T>` 加上一组方法。  
   - 你也可以写得更显式一些：  
     ```rust
     impl<T> Point<T> {
        // …
     }
     ```  
     或者等价地  
     ```rust
     impl<T> Point<T> {
        // …
     }
     ```  
     （注意：关键是 `Point<T>`，而不是写成 `Point`。）

2. `fn x(&self) -> &T`  
   - `&self` 是方法第一个参数，等同于 `self: &Point<T>`，也就说你是借用（immutable borrow）一个 `Point<T>` 来调用这方法。  
   - 返回值 `&T`，表示「一个指向 `T` 的不可变引用」。  
   - 在方法体里写 `&self.x`，就是把 `self`（一个 `&Point<T>`）解借用，取它的字段 `x`（类型是 `T`），然后对那块数据再借用一次，变成 `&T`。

3. 为什么要这么写？  
   - 如果你直接写 `fn x(self) -> T`，就是把整个 `x` 字段「移动」出 `Point`（所以调用一次后，`Point` 里的 `x` 就不见了）。  
   - 通常我们只是想「读取」一下坐标而已，不想拿走字段的所有权。这时候就用 `&self` + `-> &T`，既不克隆也不移动，编译器会在背后保证借用的生命周期安全。

4. 调用示例

   ```rust
   fn main() {
       let p = Point { x: 3, y: 4 };
       // p 是一个 Point<i32>，我们借用 p 来调用 x()
       let x_ref: &i32 = p.x();
       println!("x = {}", x_ref);
       // p 仍然完整地拥有 x 和 y
   }
   ```

5. 补充：关联函数 vs 实例方法  
   - **关联函数**（associated function）是指 `impl` 里没有 `self` 参数的函数，类似其它语言的静态方法：  
     ```rust
     impl<T> Point<T> {
         fn new(x: T, y: T) -> Point<T> {
             Point { x, y }
         }
     }
     ```  
   - **实例方法**（method）则带 `&self`、`&mut self` 或 `self`，需要在对象上调用。

---

小结：

- `impl<T> Point<T>`：为任意类型 `T` 的 `Point<T>` 添加方法。  
- `fn x(&self) -> &T`：使用不可变借用读取字段，不会夺走所有权。  
- `&self.x`：先从 `&Point<T>` 拿到字段，再对字段做一次借用返回 `&T`。  

这个模式在 Rust 中非常常见：你既不想拷贝（clone）也不想移动字段，就用 `&self` + `&field` 把一个「引用」交出去。
