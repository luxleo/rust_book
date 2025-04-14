fn statement_and_expression() {
    // let x = (let y = 6); this line is not valid because statement does not return value;
    let x = {
        // this block is expression with final returned value
        let y = 1;
        //  Expressions do not include ending semicolons.
        // If you add a semicolon to the end of an expression,
        // you turn it into a statement, and it will then not return a value
        y + 1
    };
}

fn functions_with_return_value() {
    let x = five();
    println!("five return value of x = {x}");
}
fn five() -> u8 {
    5
}