fn normal() {
    let w = 10;
    let h = 5;
    println!("area of square is {}", area_normal(w, h));
}
fn area_normal(width: u16, height: u16)->u16 {
    width * height
}

fn refactoring_with_tuple() {
    let square = (10, 5);
    println!("area of square is {}", area_tuple(square));
}

fn area_tuple(square : (u32,u32)) -> u32 {
    square.0 * square.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn refactoring_with_struct() {
    let rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("area of square is {}", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

pub fn show_rect() {
    let rect1 = Rectangle {
        width: 10,
        height: 5,
    };
    dbg!(&rect1); // dbg!()는 소유권을 가져감.
    println!("rect 1 is {rect1:#?}"); // {:?} 는 borrow 형식으로 빌려간다.
    println!("simple rect1 {rect1:?}");
}