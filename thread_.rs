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
