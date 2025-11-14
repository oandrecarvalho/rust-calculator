

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum(){
        assert_eq!(sum(3, 4), Ok(7))
    }
}