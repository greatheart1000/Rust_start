// 导入标准库中的 Rc 智能指针
use std::rc::Rc;

// 导入 std::fmt::Debug 以便打印（可选，但通常有助于调试）
// use std::fmt::Debug;

// 定义 List 枚举来模拟链表
// 为了能够使用 Rc<List>，我们需要让 List 枚举能够被引用计数
#[derive(Debug)]
enum List {
    // Cons 包含一个整数值和一个指向下一个节点的 Rc<List> 智能指针
    Cons(i32, Rc<List>),
    // Nil 表示列表的末尾
    Nil,
}

// 引入 List 的 Cons 和 Nil 变体，以便在 main 函数中直接使用它们
use List::{Cons, Nil};

fn main() {
    // ----------------------------------------------------------------
    // 1. 创建链表 a: 5 -> 10 -> Nil
    // Rc::new() 用于创建第一个 Rc 指针，并初始化引用计数为 1
    // 这里的 Nil 也被包裹在 Rc::new() 中，因为 Cons 变体要求 Rc<List>
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    
    // 打印 a 的强引用计数。
    // Rc::strong_count(&a) 获取 Rc 实例 a 的引用计数
    println!("count after creating a = {}", Rc::strong_count(&a));
    // 期望输出: 1

    // ----------------------------------------------------------------
    // 2. 创建 b，并克隆 a 的引用
    // Rc::clone(&a) 只是增加引用计数，不会复制数据
    let b = Cons(3, Rc::clone(&a));
    
    // 再次打印 a 的引用计数
    println!("count after creating b = {}", Rc::strong_count(&a));
    // 期望输出: 2 (因为 a 和 b 都引用了 List 结构)

    // ----------------------------------------------------------------
    // 3. 进入内部作用域，创建 c
    {
        // 创建 c，再次克隆 a 的引用
        let c = Cons(4, Rc::clone(&a));
        
        // 打印 a 的引用计数。
        println!("count after creating c = {}", Rc::strong_count(&a));
        // 期望输出: 3 (a, b, 和 c 都引用了 List 结构)
    } // 内部作用域结束，c 变量在此处被丢弃 (dropped)
      // 当 c 被丢弃时，它所持有的对 a 的 Rc 引用计数自动减 1。

    // ----------------------------------------------------------------
    // 4. 内部作用域结束后，检查引用计数
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // 期望输出: 2 (因为 c 已经不在作用域内)
}
