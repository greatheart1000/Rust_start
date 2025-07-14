use std::thread;
use std::time::Duration;
//多线程的例子 
fn main(){
    let handle =thread::spawn(||{
        for i in 1..10{
            println!("hi number {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap(); //等上面的线程运行完了 再运行下面的线程 
    for i in 1..5{
        println!("hi number main thread {}",i);
        thread::sleep(Duration::from_millis(1));
    }
    
}

在 Rust 中，`thread::spawn` 会在后台开启一个新线程，但它**不会**自动阻塞或等待主线程；主线程如果执行完 `main` 函数，就会直接退出整个进程，后台线程也就被一并终止了。
在你的例子里：

```rust
fn main() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("the spawn is {:?}", v);
    });
    // ↓ 如果去掉这行：
    handle.join().unwrap();
}
```

- `.join()` 的作用就是阻塞（block）当前线程，直到子线程运行完毕并返回结果。
- 如果你去掉 `handle.join().unwrap();`，主线程跑到 `main` 尾就结束了，进程立刻退出，往往子线程连打印的机会都没给到。

你可以做两件事来“留住”主线程，等子线程跑完：

1. **调用 `join`**  
   最直接、推荐的做法：
   ```rust
   let handle = thread::spawn(move || {
       println!("the spawn is {:?}", v);
   });
   handle.join().unwrap();  // 等子线程结束
   ```

2. **临时睡一会儿**（不推荐，只能用于演示）  
   ```rust
   let handle = thread::spawn(move || {
       println!("the spawn is {:?}", v);
   });
   std::thread::sleep(std::time::Duration::from_millis(10));
   ```
   这样主线程睡 10ms，子线程就有机会跑完，但这不是严谨的同步方式。

——

**总结**：在 Rust 中，主线程退出会直接终止整个进程，包括所有子线程。想让子线程有机会执行完，必须显式 `.join()`（或其他同步手段）来等待它。
