//Variables and Mutability
fn variables_and_mutability_1() {
    let x = 5;
    println!("value of x is {x}");
    // x = 6; // following line is not possible
    println!( "value of x is {x}");
}
fn variables_and_mutability_2() {
    let mut x = 5;
    println!("value of x is {x}");
    x = 6;
    println!("value of x is {x}");
}

//Constants
/**
Like immutable variables, constants are values that are bound to a name and are not allowed to change,
but there are a few differences between constants and variables.

First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.
*/
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // const can not accompany with 'mut' keyword'

// Shadowing
/**
it's not reallocating, but create whole different variable with same name
*/
fn shadowing() {
    let mut x = 5;
    println!("value of x is {x}");
    let mut x = 10;
    println!("value of x is {x}");
    {
        let mut x = 3;
        println!("value of x is {x}");
    }
}

fn shadowing_2() {
    let x = 3;
    println!("value of x is {x}");
    {
        let x= x*2;
        println!("value of x is {x}");
    }
    let x = "good";
    println!("value of x is {x}");
}

pub fn execute() {
    shadowing_2();
}
