use crate::chap7::human::{Human, Race};

/**
private mod 의 method 에는 접근하기 어렵다.
*/
pub fn recruit() {
    let human = Human::new(Race::Asian, String::from("Lee"), 10);
}