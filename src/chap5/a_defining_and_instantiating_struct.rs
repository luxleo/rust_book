struct User {
    name: String,
    email: String,
    age: u8
}

fn build_user(name: String,email: String, age: u8) -> User {
    User {
        name, email,age
    }
}

pub fn instantiate_user() {
    let u1 = build_user(String::from("Jack"), String::from("lux@gmail.com"), 25);
    let mut u2 = build_user(String::from("Jack"), String::from("lux2@gmail.com"), 25);

    // u1.name = String::from("Paul"); -> mut 으로 정의해야 필드 변경가능
    u2.email = String::from("another@gmail.com");
    println!("u1.name: {}", u1.name);
}

pub fn instantiate_from_another_instance() {
    let u1 = build_user(String::from("Jack"), String::from("lux@gmail.com"), 25);
    let u2 = User {
        name: String::from("Paul"),
        ..u1
    };
    println!("u1.name: {}", u1.name); // possible
    // /println!("u1.email: {}", u1.email); // impossible
}

// Tuple Struct
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

fn destruct_tuple_struct() {
    let p = Point(0, 0, 0);
    let c = Color(0, 0, 0);
    let Point(x,y,z) =p;
    let Color(r,g,b) = c;
}
// Unit-Like Struct without any field
struct AlwaysEqual;

// Let’s say you try to store a reference in a struct without specifying lifetimes
// , like the following; this won’t work: