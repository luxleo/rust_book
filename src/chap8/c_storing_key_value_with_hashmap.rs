use std::collections::HashMap;

// creating and access hash map
pub fn create_and_access() {
    let mut scores = HashMap::new();
    scores.insert(String::from("A"), 10);
    scores.insert(String::from("B"), 20);

    let team_name = String::from("A");
    // hash_map.get() returns Option<T>
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // iterate
    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }
}
// Hash Maps and Ownership
pub fn ownership_move() {
    let field_key = String::from("field_key");
    let field_value = String::from("field_value");
    let mut mp = HashMap::new();
    mp.insert(field_key, field_value);
    // println!("{field_key} {field_value}") -> mp.insert()의 인자로 넘기는 순간 이미 소유권이 옮겨졌다
}

// Updating a Hash Map
pub fn update_test() {
    // overwriting value
    let mut mp = HashMap::new();
    mp.insert("A", 10);
    mp.insert("A", 20);
    println!("{mp:?}"); // -> {"A": 20}

    // Adding a Key and Value Only If a Key Isn’t Present
    let mut scores = HashMap::new();
    scores.insert(String::from("A"), 10);
    scores.insert(String::from("B"), 20);

    scores.entry(String::from("C")).or_insert(30);
    scores.entry(String::from("A")).or_insert(100); // overwrite 되지 않는다.
    println!("{scores:?}"); // -> {"A": 10, "C": 30, "B": 20}
}
//Updating a Value Based on the Old Value
pub fn change_test() {
    let text = "hello world wonderful world";
    let mut cnt = HashMap::new();
    for word in text.split_whitespace(){
        let count = cnt.entry(word).or_insert(0);
        *count +=1; //  The or_insert method returns a mutable reference (&mut V) to
    }
}