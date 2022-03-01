// Program to understand mutability of variables

fn main() {
    //let s = String::from("Hello");

    let mut sm = String::from("Hello mutable");

    //s.push_str(", world");
    sm.push_str(", world");
    println!("{}", sm);
}
