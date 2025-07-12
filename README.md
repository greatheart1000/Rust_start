# Rust_start
## RUST语言入门
### Rust学习资料 https://kaisery.github.io/trpl-zh-cn/ch01-02-hello-world.html
wsl 环境下 安装Rust curl -4 --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh <br>
配置环境变量 source $HOME/.cargo/env  <br>
验证安装：   <br>
rustc --version <br>
cargo --version <br>
cd /mnt/d/project/Rust_project/lession01  <br>
使用 cargo init 命令。这个命令会在当前目录初始化一个新的 Cargo 项目，并创建 Cargo.toml 文件和 src 目录 <br>
可以使用 cargo new 创建项目。 <br>
可以使用 cargo build 构建项目。 <br>
可以使用 cargo run 一步构建并运行项目。 <br>
可以使用 cargo check 在不生成二进制文件的情况下构建项目来检查错误。 <br>


在 Rust 里，`Option<T>` 是一个枚举，用来表示「可能有值」或「可能无值」的情况：

```rust
enum Option<T> {
    None,
    Some(T),
}
```

- `Some(x)` 表示有一个类型为 `T` 的值 `x`  
- `None`    表示没有值

---

## `Some` 的常见用法

1. 创建一个可选值  
   ```rust
   let config_max = Some(3u8);  // Option<u8>，它就是 Some(3)
   let config_min: Option<u8> = None;
   ```

2. 用 `match` 或 `if let` 解构取值  
   ```rust
   match config_max {
       Some(max) => println!("最大值是：{}", max),
       None      => println!("没有配置最大值，使用默认"),
   }

   // 或者：
   if let Some(max) = config_max {
       println!("最大值是：{}", max);
   } else {
       println!("没有配置最大值，使用默认");
   }
   ```

3. 用 `Option` 自带的方法优雅地取值／提供默认  
   ```rust
   // 如果有值就返回该值，否则返回默认 10
   let max = config_max.unwrap_or(10);
   println!("max = {}", max);
   ```

4. 链式调用，安全地对内部值做转换  
   ```rust
   // 假设我们想把 u8 * 2，然后还是一个 Option<u8>
   let doubled = config_max.map(|v| v * 2);
   // doubled: Option<u8> == Some(6)
   ```

5. 链式取更深层的 Option（flat_map）  
   ```rust
   // 假设我们有一个函数：fn parse(s: &str) -> Option<i32>
   // 想先获取环境变量，再解析成数字：
   fn parse(s: &str) -> Option<i32> { s.parse().ok() }

   let result: Option<i32> =
       std::env::var("PORT").ok()   // Option<String>
                   .and_then(|s| parse(&s));  // Option<i32>
   ```

6. 安全地取到引用或可变引用  
   ```rust
   let mut opt = Some(String::from("hello"));
   if let Some(s) = opt.as_mut() {
       // s: &mut String
       s.push_str(" world");
   }
   println!("{:?}", opt); // Some("hello world")
   ```

---

### 小结

- `Some(x)` 就是把 `x` 包在 `Option` 里，代表“这里有个值”。  
- `None` 代表“这里什么都没有”。  
- 通过 `match`、`if let`、或者 `Option` 自带的方法（`unwrap_or`、`map`、`and_then`、`as_ref`、`as_mut`…）来安全地获取或操作这个值，避免 `null` 带来的空指针崩溃。
- if else 表达式
fn main(){
    let num=3;
    if num>=3{
        println!("值大于3")
    }
    else {
        println!("小于3")
    }

}
'''python
fn main(){
   let mut counter=100;
   loop {
       if counter>0 {
        println!("contuner :{}",counter);
        counter-=1;
       }
       else {
        break;
       }
   }
} '''


### Rust的代码组织 <br>
·代码组织主要包括：
－哪些细节可以暴露，哪些细节是私有的
－作用域内哪些名称有效
### 模块系统:
－Package（包）：Cargo 的特性，让你构建、测试、共享crate  <br>
-Crate（单元包）：一个模块树，它可产生一个library或可执行文件 <br>
－Module（模块）、Use：让你控制代码的组织、作用域、私有路径 <br>
-Path（路径）：为struct、function或module等项命名的方式 <br>
