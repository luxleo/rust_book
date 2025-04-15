use std::io;

fn first_word_len(s : &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}

pub fn invalid_length() {
    let mut s = String::from("hello world");
    let word = first_word_len(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!
}

fn first_word(s : &str) -> &str{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

// borrow compile error 일어남.
// pub fn valid_length() {
//     let mut s = String::from("hello world");
//     let first_word = first_word(&s);
//     s.clear();
//     println!("first word is {first_word}");
// }


// &str 말고 다른 타입도 슬라이스 가능하다.
fn slice_arr(arr : &[i32],s_idx: u32, e_idx:u32) -> &[i32] {
    &arr[s_idx as usize..e_idx as usize]
}

pub fn show_sliced_array() {
    let arr = [1, 2, 3, 4, 5, 6];
    let mut s = String::new();
    println!("type start index");
    io::stdin().read_line(&mut s).expect("TODO: panic message");
    let s : u32= s.trim().parse().unwrap();

    let mut e = String::new();
    println!("type end index");
    io::stdin().read_line(&mut e).expect("TODO: panic message");
    let e : u32= e.trim().parse().unwrap();

    let sliced = slice_arr(&arr, s, e);
    for el in sliced{
        println!("{}, ", el);
    }
}