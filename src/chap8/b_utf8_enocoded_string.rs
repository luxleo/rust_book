// Creating a New String
fn ways_to_create_string() {
    let s1 = String::new();
    let s2 = "string literal".to_string();
}
pub fn update_string() {
    let mut s1 = String::from("hello");
    s1.push_str(", world"); // borrow from s1
    println!("{s1}");
    s1.push('!'); // push takes only one character
    println!("{s1}");

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    // if we need to concatenate multiple strings, the behavior of the '+' operator gets unwieldy
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + &s2 + &s3; -> 해당라인 uncomment 하면 밑의 라인 실행할 수 없다.
    let s = format!("{}-{}-{}", s1, s2, s3);
}

// Indexing string - not recommended
pub fn indexing_string() {
    let s = "Здравствуйте";

    // slicing string is very dangerous because of encoding
    let answer = &s[..2];
    println!("{answer}");

    // iterate on string as char -> very recommendable
    for c in "Зд".chars(){
        println!("{}", c);
    }
    // iterate on string as byte
    for b in "Зд".bytes(){
        println!("{}", b);
    }
}