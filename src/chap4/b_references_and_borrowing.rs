// reference and borrowing
fn calc_length(s : &String) -> usize {
    s.len()
}
// Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the value is not dropped.
fn borrowing_through_reference() {
    let s = String::from("hello");
    println!("{}", calc_length(&s));
    println!("{}", s); // ownership is not taken by calling function
}

// mutable reference
fn change(s: &mut String) {
    s.push_str(", world");
}
fn valid_change() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

// 'error[E0499]: cannot borrow `s` as mutable more than once at a time' will be occurred
// rust use this rule to prevent data race or 'racing condition'

// A data race is similar to a race condition and happens when these three behaviors occur:
//
// Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There’s no mechanism being used to synchronize access to the data.
fn invalid_change() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
}

pub fn valid_change_with_multiple_mutable_references1() {
    let mut s= String::from("hello");
    let r1 = &mut s;
    // below is also not possible because first mutable reference's lifespan extends through block
    // {
    //     let r2 = &mut s;
    // }
    change(r1);
    let r2 = &mut s;
}
pub fn valid_change_with_multiple_mutable_references2() {
    let mut s= String::from("hello");
    { let r1 = &mut s; }
    // another mutable reference possible because first reference lifespan ends, end eliminated with 'drop' function
    let r2 = &mut s;
}

pub fn valid_multiple_references() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // because two immutable reference lifespan ends above, declare another mutable reference is possible
    let r3 = &mut s;
}
//Dangling References
// dangling pointer—a pointer that references a location
// in memory that may have been given to someone else—by freeing some
// memory while preserving a pointer to that memory.

// fn dangle() -> &String { // dangle returns a reference to a String
//
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
//This works without any problems. Ownership is moved out, and nothing is deallocated.


