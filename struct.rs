// program to use and understand struct
use std::io;

struct user_ip(i32, i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someonename123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let user1_ip = user_ip(192, 128, 0, 1);

    println!("Enter your email: ");
    let mut new_email = String::new();

    io::stdin()
        .read_line(&mut new_email)
        .expect("Failed to read line.");

    println!("Enter your username: ");
    let mut new_user = String::new();

    io::stdin()
        .read_line(&mut new_user)
        .expect("Failed to read line.");

    let user3 = build_user(new_email, new_user);

    //println!(" {}\n {}\n {}\n {}", user2.active, user2.username, user2.email, user2.sign_in_count);
    println!("\n {}\n {} {} {}", user3.active, user3.username, user3.email, user3.sign_in_count);
}
