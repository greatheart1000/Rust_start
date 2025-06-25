//查看字符串在哪个位置的方法
fn get_index(s:&str)->Option<usize>{
    for (index,item) in s.char_indices() {
        if item=='b' {
            return Some(index);
        }
    }
    None
}
fn main(){
    let s="ghbd";
    println!("{:?}",get_index(s));

}


在 Rust 里，`&str` 和 `String`（以及你用 `String::from("…")` 构造的类型）虽然都代表“文本”，但它们在所有权、存储位置、可变性等方面有本质区别：

1. **`&str`（string slice）**  
   - 是一个 **借用**（reference），底层只是一个指针 + 长度：它指向某段已经存在的 UTF-8 字节序列，自己不拥有那段内存。  
   - 不可变，也不能增删字符。  
   - 可以指向  
     - 字符串字面量（`let s: &str = "hello";`，这段字面量在程序二进制的 `.rodata` 段，具有 `'static` 生命周期）  
     - 也可以从 `String` “借用”出来：  
       ```rust
       let mut owned = String::from("hello");
       let slice: &str = &owned;  // 从 String 借用出一个 &str
       ```

2. **`String`（可 grow 的堆分配字符串）**  
   - 是一个 **拥有者**（owned），底层是一个 `Vec<u8>`：指针 + 长度 + 容量，都在堆上分配。  
   - 可以动手修改、拼接、push、pop 等。  
   - 可以随时被 `&` 或 `&mut` 借用成 `&str`／`&mut str`。

---

### 为什么函数参数写成 `fn foo(s: &str)` 更灵活

```rust
fn get_index(s: &str) -> Option<usize> { /* ... */ }
```

- 此时你既可以直接传字符串字面量：  
  ```rust
  get_index("hello world");
  ```
- 也可以传一个 `String` 的引用：  
  ```rust
  let x = String::from("owned string");
  get_index(&x);
  ```

但如果你写成 `fn get_index(s: &String)`, 那调用就只能接收 `&String`，**没法**直接写字面量 `"abc"` ——编译器会抱错 “expected `&String`, found `&str`”。

---

### 回到你的代码示例

```rust
/// 按字节遍历，找到第一个 b
fn get_index(s: &str) -> Option<usize> {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b'b' {
            return Some(i);
        }
    }
    None
}

fn main() {
    // 这是一个 &'static str
    let s: &'static str = "helloby";
    println!("索引位置在{:?}", get_index(s));
}
```

- `s: &str` 的声明里没有问题，它是一个对静态区 `"helloby"` 的借用。  
- `get_index` 接受 `&str`，可以同时处理字面量和 `String`。

唯一要注意的地方是遍历字节时的**模式匹配**写法：

```rust
for (i, &item) in s.as_bytes().iter().enumerate() { … }
```

而 **不要** 在 `for` 里给模式加类型注解，比如写成 `(i: usize, &item: u8)` —— Rust 语法里，`for` 的模式部分不能这么写，会编译不过。只要写成上面那样，编译器会自动推断 `i: usize`，`item: u8`。

---

### 小结

- 用 `String::from("…")` 得到的是一个**拥有者**，可修改、可伸缩，存储在堆上。  
- 用 `&str` 得到的是一个**借用**，指向某段 UTF-8 数据，读不可写，存储在字面量区或堆上已有的 `String` 里。  
- 如果你只读、只需要索引／遍历，函数签名写成 `&str` 最通用；要拿到一个可变的、可持有所有权的字符串，就用 `String`。


fn get_index(s:&str)->Option<usize>{
    for (i,&item) in s.as_bytes().iter().enumerate(){
        if item==b'b'{
            return Some(i);
    }
    
}
None
}
fn main(){
    let s="helloby";
    let index = get_index(s).unwrap();
    println!("索引位置在{index}");
}

//match的用法 
fn get_index(s:&str)->Option<usize>{
    for (index,&tiem) in s.as_bytes().iter().enumerate(){
        if tiem==b'b'{
            return Some(index);
    }
    }
    None
}

fn main(){
    let strs ="hellovb";
    match get_index(strs) {
        Some(index)=>{
            println!("索引是:{}",index);
        }
        None=>{
            println!("字符串不存在");
        }
    }
}
