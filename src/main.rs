use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("猜数字游戏");
    let  secret_number = String::new();
    let a = test(secret_number);
    println!("{a}");
}
fn test(mut str:String) -> String{
    str.push_str(", test!");
    return str;
}