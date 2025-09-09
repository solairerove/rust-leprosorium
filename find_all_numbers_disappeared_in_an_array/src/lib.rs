pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut seq: Vec<i32> = [0].repeat(nums.len());

    nums.into_iter()
        .for_each(|num| seq[(num - 1) as usize] += 1);

    seq.into_iter()
        .enumerate()
        .filter(|&(_x, y)| y == 0)
        .map(|(x, _y)| (x + 1) as i32)
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(find_disappeared_numbers(vec![1, 1]), vec![2]);
    }

    #[test]
    fn no_missing_numbers() {
        assert_eq!(find_disappeared_numbers(vec![1, 2, 3, 4]), vec![]);
    }

    #[test]
    fn single_element_present() {
        assert_eq!(find_disappeared_numbers(vec![1]), vec![]);
    }

    #[test]
    fn all_same_number() {
        assert_eq!(find_disappeared_numbers(vec![3, 3, 3, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn missing_multiple_consecutive() {
        assert_eq!(find_disappeared_numbers(vec![1, 1, 1, 1]), vec![2, 3, 4]);
    }

    #[test]
    fn reverse_order() {
        assert_eq!(find_disappeared_numbers(vec![3, 2, 1]), vec![]);
    }

    #[test]
    fn duplicates_scattered() {
        assert_eq!(find_disappeared_numbers(vec![2, 2, 4, 4]), vec![1, 3]);
    }
}
