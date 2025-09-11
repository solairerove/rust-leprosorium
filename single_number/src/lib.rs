pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |prev, curr| prev ^ curr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number_basic() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn test_single_number_middle() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn test_single_number_negative() {
        assert_eq!(single_number(vec![-1, -1, -2]), -2);
    }

    #[test]
    fn test_single_number_single_element() {
        assert_eq!(single_number(vec![1]), 1);
    }

    #[test]
    fn test_single_number_large_array() {
        assert_eq!(single_number(vec![1, 2, 3, 2, 1]), 3);
    }

    #[test]
    fn test_single_number_zero() {
        assert_eq!(single_number(vec![0, 1, 1]), 0);
    }

    #[test]
    fn test_single_number_mixed_positive_negative() {
        assert_eq!(single_number(vec![-1, 2, -1, 2, 5]), 5);
    }
}
