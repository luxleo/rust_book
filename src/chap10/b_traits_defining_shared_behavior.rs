use std::fmt::Display;
use std::io::Bytes;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub title: String,
    pub author: String
}
pub struct Book {
    pub author: String,
    pub title: String
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}
impl Summary for Book {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.author)
    }
}

// default implementation
pub trait SumUp{
    fn sum_up(&self)-> f64;
    fn get_unit(&self) -> &str;
    fn show_total(&self)-> String {
        format!("total: {}", self.sum_up())
    }
}

pub struct Research {
    pub unit: String,
    pub researcher_num: u32,
    pub salary: u32,
}

pub struct FieldWork{
    pub unit: String,
    pub workforce_num: u32,
    pub salary: u32,
}
impl SumUp for Research {
    fn sum_up(&self)-> f64 {
        (self.researcher_num * self.salary ) as f64
    }
    fn get_unit(&self) -> &str {
        &self.unit
    }
}

impl SumUp for FieldWork {
    fn sum_up(&self)-> f64 {
        (self.workforce_num * self.salary) as f64
    }
    fn get_unit(&self) -> &str {
        &self.unit
    }
}

fn notify(process: &impl SumUp) -> String {
    format!("total cost of process : {}", process.show_total())
}

// verbose version of notify
fn notify_longer<T:SumUp>(process: T) -> String{
    format!("total cost of process : {}", process.show_total())
}

// multiple trait constraint
fn notify_multiple(process: &(impl SumUp + Display)) -> String{
    format!("total cost of process : {}", process.show_total())
}

fn notify_multiple_verbose<T: SumUp + Display>(process: T) -> String{
    format!("total cost of process : {}", process.show_total())
}

// clearer trait bounds with where clauses
fn notify_with_where<T,U> (process1: T, process2: U) -> String
where
    T: SumUp + Display,
    U: SumUp + Display
{
    format!("total cost of process1 : {}", process1.show_total())
}

// returning types that implement traits
fn returns_sum_up() -> impl SumUp {
    Research{
        researcher_num: 9,
        salary: 1000,
        unit: String::from("$")
    }
}

// using trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self{
        Self{x, y}
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) -> String {
        if self.x >= self.y {
            format!("x is bigger ({}, {})", self.x, self.y)
        } else {
            format!("y is bigger ({}, {})", self.x, self.y)
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn trait_test(){
        let book1 = Book {
            author: String::from("jorge vowel"),
            title: String::from("The Ore Guy"),
        };
        println!("{}", book1.summarize());
    }
}