//In other words, you can think of if let as syntax sugar
// for a match that runs code when the value matches one
// pattern and then ignores all other values.
//
// We can include an else with an if let. The block of code that
// goes with the else is the same as the block of code that would
// go with the _ case in the match expression that is
// equivalent to the if let and else.

fn if_let() {
    let target = Some(3u8);
    match target {
        Some(num ) => println!("{}", num),
        _ => (),
    }
    if let Some(3u8) = target {
        println!("{:?}", target);
    } else {
        print!("it's nothing");
    }
}

// let else
// @see https://doc.rust-lang.org/stable/book/ch06-03-if-let.html