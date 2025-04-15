#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self { // Self in current 'impl' block means Rectangle
        Rectangle {
            width: size,
            height: size
        }
    }
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}
impl Rectangle {
    fn area(self: &Self) -> u32 { // self: &Self === &self
        self.width * self.height
    }
    fn update_width(&mut self, new_width:u32) {
        self.width = new_width;
    }
    fn can_contain(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn method_test() {
    let mut s1 = Rectangle::square(3);
    let mut s2 = Rectangle::new(10, 5);

    println!("s1={s1:?}\n s2={s2:?}");
    println!("can s1 contain s2={}", s1.can_contain(&s2));
    println!("can s2 contain s1={}", s2.can_contain(&s1));

    s1.update_width(s2.width+1);
    println!("s1={s1:?}\n s2={s2:?}");
    println!("can s1 contain s2={}", s1.can_contain(&s2));
    println!("can s2 contain s1={}", s2.can_contain(&s1));
}