
pub fn square(x: i16) -> i32 {
    (x as i32) * (x as i32)
    }


pub fn find_min<T: Ord>(v: Vec<T>) -> T {

    

    v.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
use crate::square;
use crate::find_min;
#[test]
fn square_two() {

assert_eq!(square(2), 4);
}
#[test]
fn square_10k() {
    assert_eq!(square(10000),100000000);
 }
#[test]
 fn test_vec(){
    assert_eq!(find_min(vec![1,2,3,4,5,6]),1); 
 }
}