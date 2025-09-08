use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    let vector_size = nums.len();
    for num in nums {
        // if set.contains(&num) {
        //     return true;
        // }
        set.insert(num);
    }

    set.len() != vector_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn example_2() {
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_3() {
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }

    #[test]
    fn empty_array() {
        assert!(!contains_duplicate(vec![]));
    }

    #[test]
    fn single_element() {
        assert!(!contains_duplicate(vec![42]));
    }

    #[test]
    fn large_array_with_duplicates() {
        let mut nums: Vec<i32> = (1..=1000).collect();
        nums.push(500); // Add duplicate
        assert!(contains_duplicate(nums));
    }

    #[test]
    fn max_min_values() {
        assert!(contains_duplicate(vec![i32::MAX, i32::MIN, i32::MAX]));
        assert!(!contains_duplicate(vec![i32::MAX, i32::MIN]));
    }

    #[test]
    fn all_same_elements() {
        assert!(contains_duplicate(vec![1, 1, 1, 1, 1]));
    }
}
