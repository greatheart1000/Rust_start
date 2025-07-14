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


创建 Channel
·使用mpsc:channel 函数来创建Channel
－mpsc表示 multiple producer,single consumer（多个生产者、一个消费者）
－返回一个tuple（元组）：里面元素分别是发送端、接收端
(例子)


use std::sync::mpsc;
use std::thread;

fn main(){
    // mcsc 来创建channel 通道  这个元组是两个 tx: 发送端 rx: 接受端 
    let (tx,rx) =mpsc::channel();
    thread::spawn(move ||{
        let val =String::from("hi hi");
        tx.send(val).unwrap();
    });//创建了一个线程，move关键词 让发送端有所有权 
    // rx 接受端 recv 接受 
    let received =rx.recv().unwrap();
    println!("got:{}",received);
}


use std::sync::Mutex;

fn main() {
    // 1. 创建一个 Mutex，初始内部值为 5
    let m = Mutex::new(5);

    // 2. 在一个作用域内，通过 lock() 获取互斥锁的锁 guard
    {
        // lock() 返回 Result<MutexGuard<i32>, _>，unwrap() 用于处理可能的中毒（poison）
        let mut num = m.lock().unwrap();
        // 通过 MutexGuard 解引用，就可以像操作普通 &mut i32 一样修改内部值
        *num = 6;
        // 作用域结束时，num（即 MutexGuard）被 drop，自动释放锁
    }

    // 3. 打印整个 Mutex，Debug 输出会显示其内部当前的值
    println!("the number is {:?}", m);
}
注意 m本来在作用域外面 但是有了这个 lock().unwrap() 可以共享状态  然
the number is Mutex { data: 6, poisoned: false, .. }


当你写下这两行：

```rust
let mut num = m.lock().unwrap();
//  ↑─── num 的类型是 MutexGuard<i32>，内部持有对 5 的独占访问
*num = 6;
```

实际发生了什么？
1. `m.lock().unwrap()`  
   - 拿到了一把“钥匙”——`MutexGuard<i32>`。  
   - 这个 guard 同时包含了对 `m` 内部数据的独占访问权限，以及当它离开作用域时要自动释放锁的“责任”。
2. `MutexGuard<T>` 实现了 `DerefMut<Target = T>`  
   ```rust
   impl<T> Deref for MutexGuard<T> { /* … */ }
   impl<T> DerefMut for MutexGuard<T> { /* … */ }
   ```
   这意味着你可以把它当做 `&T`（或 `&mut T`）来读写内部的数据。
3. `*num = 6;`  
   - `*num` 相当于“解引用”这把钥匙，直接指向锁里面的那个 `i32`。  
   - 于是写 `*num = 6`，就是在锁住的位置把那个原来为 `5` 的值直接改成 `6`。
4. 作用域结束，`num`（即那把钥匙、`MutexGuard`）被 `drop`，**锁被释放**，但内部数据已经被改成了 `6`。
5. 再打印 `m` 时，你看到的就是锁里最新的值：  
   ```
   the number is Mutex { data: 6 }
   ```
如果把它分成更“指针化”的思路：
- `m` 本质上存了一个 `i32`（初始是 `5`），你平时**不能**直接拿出来改，因为它被锁保护着。
- `m.lock()` 给了你一根“指针” `num: MutexGuard<i32>`，指向那块被保护的内存。
- 用 `*num = 6`，就是往那块内存写 `6`。
- 写完后，锁一松（`MutexGuard` 掉了），但那块内存的值已经永远变成 `6` 了。
希望这样解释能帮你彻底搞清 `*num = 6` 为什么能修改到 `m` 里的数据！
