pub fn vector_define() {
    // define vector
    let mut v1: Vec<i8> = Vec::new();
    for i in 1..10{
        v1.push(i);
    }

    let v2 = vec![1,2,3,4];
    for el in &v2{
        println!("{:?}", *el);
    }
}
pub fn update_vector() {
    let mut v1 = Vec::new();
    for i in 1..10{
        v1.push(i);
    }
    for _ in 0..10 {
        v1.pop();
    }

    let mut v2 = vec![1; 20]; // macro with array initializing method
    for i in 2..20{
        v2[i] = v2[i-1] + v2[i-2];
    }

    // for의 in 문 내부에서 iter()함수 호출되고 이때 소유권이 넘어간다.
    // for el in v2{
    //     println!("{:?}", el);
    // }
    for el in &v2{
        println!("{}", el);
    }
}

// save multiple type value using enum
enum Block {
    Float(f64),
    Integer(i32),
    Text(String)
}
fn put_different_type_using_enum() {
    let v=vec![Block::Float(1.1), Block::Integer(1), Block::Text(String::from("Hello"))];
}

// get element of vector
pub fn get_element_test() {
    let mut v = vec![0;10];
    let first = v[0]; // 복사할당이다. 단순히 현재의 값만 필요하다면 이렇게 해도 무방하다.
    // let second = &v[1]; // immutable borrow occurs here

    v[0] = 1;
    // v[1] = 2; mutable borrow occurs here
    println!("{first}");
    // println!("{second}"); immutable borrow later used here

    // let does_not_exist = &v[100]; // will cause panic!
    let does_not_exist : Option<&i32> = v.get(100); // will return None
}

//Iterating Over the Values in a Vector
pub fn prac_iter_vector(){
    let mut v = vec![1; 10];
    for el in &v {
        println!("{el}");
    }
    for el in &mut v{
        *el +=2;
    }
    for el in &v {
        println!("{el}");
    }
}

