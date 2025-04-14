use std::arch::x86_64::_mm256_castsi256_si128;

/**
heap and stack : stack에는 컴파일 시점에 메모리 계산 및 할당이 가능하다. heap
은 그렇지 못하다.
stack은 copy하여 할당할 수 있다. heap은 참조형 데이터들이 저장되므로 deep copy로 복사가능하다.

rust에서 소유권 이전 대상은 stack에 있는 변수들이다.
*/
pub fn stack_data() {
    let x=3;
    let y =x;
    println!("x = {}, y = {}", x, y);
}

pub fn heap_data() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s2 = s;
    println!("{}", s2);
    // println!("{}", s); // Value used after being moved
}

fn heap_call_drop_to_clean() {
    let s = String::from("hello");
    {
        let s1 = String::from("hello world");
        println!("s1 is {}", s1);
    }
    // println!("s1 is {}", s1); 이 라인은 drop이 호출된 이후이므로 메모리에 접근할 수 없다.
}

// Ownership and functions
fn ownership_and_functions() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("is valid {s}"); ownership이 function scope로 넘어갔기 때문에 유효하지 않다.
    let x = 3;
    makes_copy(x);
    println!("is valid {x}"); // Copy trait 구현체 이므로 소유권 넘기지 않고 복사한다.
}
fn takes_ownership(str: String) {
    println!("take ownership : {str}");
}

fn makes_copy(val : i32){
    println!("makes copy val : {val}");
}

// Return values and scope
fn return_values_and_scope() {
    let taken_ownership = gives_ownership();
    let retaken_ownership = takes_and_gives_back(taken_ownership);

    println!("retaken ownership is {retaken_ownership}");
}
fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(str: String) -> String {
    println!("takes and give back : {str}");
    str
}

fn takes_and_get_length(str:String) -> (String, usize) {
    // (str, str.len()) str에서 ownership이 이동하였으므로 불가능하다. 만일 return type이 (usize,String) 이라면 가능하다.
    let len = str.len();
    (str, len)
}