use std::collections::HashMap;
use std::io;
use std::io::Read;

fn if_traits() {
    let number = 7;
    // if number {} 'rust' does not allow truthy or falsy expression, it only allows the bool
    let chaos = if number > 0 { "positive" } else { "negative" };
    // let chaos = if number > 0 { "positive" } else { -1 }; if expression should return same type
}

pub fn repetition_with_loop() {
    // returning value with loop
    let mut counter = 0;

    // loop work as 'expression'
    let acc = loop {
        counter += 1;
        if counter == 10 {
            break counter * 3;
        }
    };

    let arr = [[3;3];3];
    let mut pos = (0, 0);
    'outer: loop {
        loop {
            println!("arr[{}][{}]={}",pos.0,pos.1,arr[pos.0][pos.1]);
            if (pos.0 == 2 && pos.1 == 2) {
                break 'outer;
            }
            pos.0+=1; pos.1+=1;
        }
    }
}

fn repetition_with_while() {
    let mut number = 0;
    while number < 10 {
        number += 1;
    }
}

pub fn repetition_with_for() {
    let arr = [3; 5];
    for el in arr{
        println!("{}", el);
    }

    for i in (1..5) {
        println!("{}", arr[i]);
    };
}

pub fn fibo_client(){
    loop{
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read n");
        if n.trim() == "break" {
            break;
        }
        let n = match n.trim().parse() {
            Ok(num) => {
                if num < 0 {
                    println!("only positive number");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("type a number");
                continue;
            }
        };
        println!("result of nth fibo={}", fibo(n));
    }
}

fn fibo(x:u32) -> u32{
    if x==1 || x==2 {
        return 1;
    }
    fibo(x - 1) + fibo(x - 2)
}
pub fn fibo_cache_client() {
    loop{
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read n");
        if n.trim() == "break" {
            break;
        }
        let n = match n.trim().parse() {
            Ok(num) => {
                if num < 0 {
                    println!("only positive number");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("type a number");
                continue;
            }
        };
        let mut cache: HashMap<u32, u128> = HashMap::new();
        println!("result of nth fibo={}", fibo_cahce_upward(n,&mut cache));
    }
}
fn fibo_cache(x:u32, cache: &mut HashMap<u32, u128>) -> u128{
    if let Some(&result) = cache.get(&x) {
        return result;
    }

    let result = match x {
        0 => 0,
        1 => 1,
        _ => fibo_cache(x - 1, cache) + fibo_cache(x - 2, cache)
    };
    cache.insert(x, result);
    result
}

// TODO: 왜 100번 이상 호출 할 수 없는지 알아보기
fn fibo_cahce_upward(x:u32, cache: &mut HashMap<u32,u128>) -> &u128 {
    cache.insert(0, 0);
    cache.insert(1, 1);
    for num in (2..x+1){
        cache.insert(num,
                     cache.get(&(num - 1)).unwrap() + cache.get(&(num - 2)).unwrap());
    }
    cache.get(&x).unwrap()
}