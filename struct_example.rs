// program to understand using structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    //let rect1 = (30, 50);

    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30*scale),
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    dbg!(&rect1);

    //println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
