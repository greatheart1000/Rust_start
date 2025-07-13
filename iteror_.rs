//迭代器方法
use std::error::Error;
use std::fs;
struct Counter{
    count:u32
}

impl Counter {
    fn new()->Counter{
        Counter {count:0}
    }
}
//关联函数 Iterator 迭代器 这个Iterator有next方法 
impl Iterator for Counter {
    type Item =u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count<5{
            self.count+=1;
            Some(self.count)
        }
        else {
            None
        }
    }
}
#[test]
fn calling_next_directly(){
    let mut counter =Counter::new(); //定义了一个可变的结构体
    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);

}

