pub fn sum(a: i32, b: i32) -> i32 {
    a+b
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_positives(){
        assert_eq!(sum(3, 4), 7)
    }
}