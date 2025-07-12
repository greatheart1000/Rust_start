//use 关键字 
mod front_of_house{
    mod hosting{
        pub fn adding_to_waitlist(){}
    }
}
use crate::front_of_house::hosting::add_to_waitlist;

pub fn use_fn(){
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
