#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
在 Rust 中，  
```rust
#[derive(Debug)]
```
是一个「派生属性」（derive attribute），它告诉编译器自动为后面的类型生成对 `std::fmt::Debug` trait 的实现。  
---
## 为什么要用 `Debug`

- `Debug` 是给开发者看的格式化输出，用来快速打印结构体内部字段。  
- 有了 `Debug`，你就可以在 `println!`／`eprintln!` 中用 `{:?}`（或更漂亮的 `{:#?}`）来打印：  

  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  fn main() {
      let r = Rectangle { width: 30, height: 50 };
      println!("{:?}", r);
      // 输出: Rectangle { width: 30, height: 50 }
  }
  ```

如果不加 `#[derive(Debug)]`，编译器就不会帮你实现 `fmt::Debug`，你在尝试 `println!("{:?}", …)` 时会报错：

```
error[E0277]: `Rectangle` doesn't implement `Debug`
  --> src/main.rs:XX:YY
   |
XX |     println!("{:?}", r);
   |              ^^^^^ `Rectangle` cannot be formatted using `{:?}`
```

---

## 派生更多常用 trait

Rust 标准库也支持一键派生其他常用 trait，例如：

- `Clone`, `Copy`  
- `PartialEq`, `Eq`, `PartialOrd`, `Ord`  
- `Hash`  

用法都是类似的：

```rust
#[derive(Clone, Debug, PartialEq)]
struct Foo { … }
```

这样就免去了手写这些 trait 实现的繁琐。
