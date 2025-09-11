const LUT: &[i32] = &[
    0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
    17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578,
    5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155, 165580141, 267914296,
    433494437, 701408733, 1134903170, 1836311903,
];

pub fn climb_stairs(n: i32) -> i32 {
    LUT[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs_base_case_1() {
        assert_eq!(climb_stairs(1), 1);
    }

    #[test]
    fn test_climb_stairs_base_case_2() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn test_climb_stairs_example_3() {
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn test_climb_stairs_4() {
        assert_eq!(climb_stairs(4), 5);
    }

    #[test]
    fn test_climb_stairs_5() {
        assert_eq!(climb_stairs(5), 8);
    }

    #[test]
    fn test_climb_stairs_6() {
        assert_eq!(climb_stairs(6), 13);
    }

    #[test]
    fn test_climb_stairs_larger() {
        assert_eq!(climb_stairs(10), 89);
    }

    #[test]
    fn test_climb_stairs_medium() {
        assert_eq!(climb_stairs(7), 21);
    }
}
