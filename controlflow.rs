// This program explores control flow 

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => {
            println!("Now that is a nickel!");
            5
        }
        Coin::Dime => {
            println!("On the dime!");
            10
        }
        Coin::Quarter => {
            println!("Quarter eh?");
            25
        }
    }
}

fn main() {
    println!("Hello, world");
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter);
}