enum Coin {
    first,
    second,
    third,
    four
}

fn value_cents(coin:Coin)->u8{
    match coin {
        Coin::first=>1,
        Coin::second=>2,
        Coin::third=>3,
        Coin::four=>4,

    }
}
fn main(){
    let one =Coin::first;
    println!("one的面值是:{}",value_cents(one));
}
