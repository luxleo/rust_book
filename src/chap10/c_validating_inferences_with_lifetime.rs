// lifetime annotation
// purpose: to prevent dangling reference from arise

// 1. lifetime annotation with function signature

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 2. thinking in terms of lifetime

/**
this function is valid because returned value's lifetime
is same as parameter
*/
fn longest_valid<'a> (x: &'a str, y: &str) -> &'a str {
    x
}

/**
this function is invalid because returned value's lifetime
does related with parameter's lifetime -> COMPILE ERROR OCCUR
*/
// fn longest_invalid<'a> (x: &'a str, y: &str) -> &'a str {
//     let result = String::from("really long string ~~~");
//     result.as_str()
// }

// 3. lifetime annotation with in struct definition

struct MessageContainer<'a> {
    message : &'a str
}

/**
this function will compile successfully.
even though the 'message' variable exist before declaration of 'message_container'
compiler can notice that lifetime of instance of MessageContainer and 'message' variable
does not conflict
*/
fn allocate_message() {
    let message = String::from("raw message");
    let message_container = MessageContainer { message: &message };
}

// 4. lifetime elision

// first rule
//      compiler assign lifetime parameter to each parameter of that's reference
// fn foo<'a> (a: &'a u32) : one lifetime parameter of one parameter
// fn foo<'a,'b> (a: &'a i32, b: &'b i32) : two lifetime parameter for two parameter

// second rule
//      if there is only one input lifetime parameter, that lifetime parameter assigned to
//      all output lifetime parameter

// third rule
//      if there are multiple lifetime parameter and one of them is &self or &mut self
//      the lifetime parameter of self is assigned to output lifetime parameter

// 5. lifetime annotation in method definitions
impl <'a> MessageContainer<'a> {
    /**
    there are two lifetime parameter because function have two parameter.
    but one of them is &self so output's lifetime parameter is same as instance
    */
    fn announce_and_return_message(&self, message: &str) -> &str{
        println!("{}", message);
        self.message
    }
}

// 6. the static lifetime
// static lifetime is one special lifetime, which denotes that affected reference can
// live for the entire duration of program

// let s : &'static str = "I have a static lifetime";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_usage() {
        let result;
        let str1 = String::from("Hello");
        {
            let str2 = String::from("World!!!");
            result = longest(&str1, &str2);
            assert_eq!(result, str2.as_str());
        }
    }
    #[test]
    fn invalid_usage() {
        let x = String::from("hello");
        let result;
        {
            let y = String::from("world!!!");
            result = longest(&x, &y);
        }
        println!("result = {result}");
    }

}