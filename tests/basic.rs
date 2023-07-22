#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_example2() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(3, 3);
    }
}
