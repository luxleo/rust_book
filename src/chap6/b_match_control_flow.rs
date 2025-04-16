// Matches Are Exhaustive
enum Asian {
    Korean,
    Chinese,
    Japanese
}
enum Skin {
    Black,
    Yellow(Asian),
    White
}
fn people_skin_color(skin: Skin) -> String{
    match skin {
        Skin::Black => String::from("black"),
        Skin::Yellow(state) => { // bind value with 'state' variable
            println!("{}'s skin is yellow", state.to_string());
            let mut ret = String::from("yellow skin from ");
            ret.push_str(&state.to_string());
            ret
        }
        Skin::White => String::from("white")
    }
}

fn plus_two(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        None => None
    }
}

//Catch-all Patterns and the _ Placeholder
fn only_two_and_seven(x: i32) -> i32 {
    match x {
        2=>2,
        7=>7,
        _=>-10
    }
}