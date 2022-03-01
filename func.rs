// This program is to try out return functions

fn plus_one(x: i32) -> i32 {
    x+1 //no semicolon as this func returns this val
}

fn main() {
    let x = 5;
    let y = plus_one(x);
    println!("The value of x is: {}", y);
}
