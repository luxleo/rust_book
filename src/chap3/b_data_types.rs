use std::io;

pub fn tuple_types(){
    let tup = (500, 3.14, "hello");
    println!("type index ranged from 1 to 3");
    loop {
        let mut index = String::new(); // must be declared in loop block
        io::stdin().read_line(&mut index) // read_line appends to existing variable!!
            .expect("Failed to read line");

        let index = match index.trim().parse() {
            Ok(num) => {
                if( num < 1 || num > 3) {
                    println!("type a number ranged in 1 to 3");
                    continue;
                }
                println!("your guess is {num}");
                num
            },
            Err(_) => {
                println!("Please type a number, you type {index}");
                continue;
            }
        };
        match index {
            1 => println!("result is {}",tup.0),
            2 => println!("result is {}",tup.1),
            3 => println!("result is {}",tup.2),
            _ => {break;}
        };
        break;
    }
}

pub fn arr_types(){
    let arr = [3; 10]; // initialize array with specifying initial value, and size
    let mut idx = 0;
    while idx < arr.len(){
        println!("arr[{idx}]={} ",arr[idx]);
        idx+=1;
    }
}
