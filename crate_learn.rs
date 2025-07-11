//crate mod 以及绝对路径 相对路径 
mod front_house{
    mod hosting {
        fn add_to_waitlist(){}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}

        fn take_payment() {}
    }
}
pub fn eat_res(){
    //绝对路径
    crate::front_house::hosting::add_to_waitlist();
    //相对路径
    front_house::hosting::add_to_waitlist();
}

fn main() {
    println!("Hello, world!");
}
