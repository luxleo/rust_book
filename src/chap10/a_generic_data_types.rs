// in function definitions
pub fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut result = &list[0];
    for el in list {
        if el > result {
            result = el;
        }
    }
    result
}

// in struct definition
struct UniPoint<T>{
    x:T,
    y:T
}
struct Point<T,U>{
    x: T,
    y: U
}

// in method definition

impl<T> UniPoint<T> {
    fn show_x(&self) -> &T{
        &self.x
    }
}
//specify constraints on generic types
impl UniPoint<f32> {
    fn show_float_x(&self) -> &f32 {
        &self.x
    }
}

// mix up struct and method signature
struct Point2D<X1,Y1>{
    x: X1,
    y: Y1
}

impl<X1,Y1> Point2D<X1,Y1>{
    fn mix_up<X2,Y2>(self, other: Point2D<X2,Y2>) -> Point2D<X1,Y2>{
        Point2D{
            x: self.x,
            y: other.y
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn find_largest_test() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = find_largest(&number_list);
        assert_eq!(*result, 100);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = find_largest(&char_list);
        assert_eq!(*result, 'y');
        println!("The largest char is {}", result);
    }

    #[test]
    fn mix_up_test() {
        let p1 = Point2D { x: 1, y: 2.0 };
        let p2 = Point2D { x: 'a', y: 'b' };
        let p3 = p1.mix_up(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
