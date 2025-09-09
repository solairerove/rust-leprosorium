pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut sum = n * (n + 1) / 2;
    for val in nums {
        sum -= val;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn missing_zero() {
        assert_eq!(missing_number(vec![1]), 0);
    }

    #[test]
    fn missing_at_end() {
        assert_eq!(missing_number(vec![0, 1, 2]), 3);
    }

    #[test]
    fn missing_in_middle() {
        assert_eq!(missing_number(vec![0, 1, 3, 4]), 2);
    }

    #[test]
    fn missing_at_beginning() {
        assert_eq!(missing_number(vec![1, 2, 3]), 0);
    }

    #[test]
    fn single_element_zero() {
        assert_eq!(missing_number(vec![1]), 0);
    }

    #[test]
    fn single_element_one() {
        assert_eq!(missing_number(vec![0]), 1);
    }

    #[test]
    fn large_array() {
        let mut nums: Vec<i32> = (0..=99).collect();
        nums.remove(50); // Remove 50
        assert_eq!(missing_number(nums), 50);
    }

    #[test]
    fn unordered_array() {
        assert_eq!(missing_number(vec![4, 1, 2, 0]), 3);
    }

    #[test]
    fn reverse_order() {
        assert_eq!(missing_number(vec![5, 4, 3, 2, 0]), 1);
    }
}
