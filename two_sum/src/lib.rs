pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

// AI generated tests
#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn example_1() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn example_2() {
        let result = two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_3() {
        let result = two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn negative_numbers() {
        let result = two_sum(vec![-1, -2, -3, -4, -5], -8);
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn mixed_positive_negative() {
        let result = two_sum(vec![-3, 4, 3, 90], 0);
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn zero_target() {
        let result = two_sum(vec![0, 4, 3, 0], 0);
        assert_eq!(result, vec![0, 3]);
    }

    #[test]
    fn large_numbers() {
        let result = two_sum(vec![1000000, 999999, 1000001], 2000001);
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn solution_at_end() {
        let result = two_sum(vec![1, 2, 3, 4, 5, 6], 11);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn duplicates_different_indices() {
        let result = two_sum(vec![5, 5, 5, 15], 10);
        assert_eq!(result, vec![0, 1]);
    }
}
