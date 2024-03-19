pub fn is_odd(val: usize) -> bool {
    (val ^ 1) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_test() {
        assert!(is_odd(1));
        assert!(is_odd(3));
        assert!(is_odd(5));
        assert!(is_odd(7));
        assert!(is_odd(9));
    }

    #[test]
    fn even_test() {
        assert!(!is_odd(0));
        assert!(!is_odd(2));
        assert!(!is_odd(4));
        assert!(!is_odd(6));
        assert!(!is_odd(8));
    }
}
