pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut profit = 0;
    for i in 1..prices.len() {
        if prices[i] < buy {
            buy = prices[i];
        } else if prices[i] - buy > profit {
            profit = prices[i] - buy;
        }
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn test_example_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }

    #[test]
    fn test_single_price() {
        let prices = vec![5];
        assert_eq!(max_profit(prices), 0);
    }

    #[test]
    fn test_two_prices_profit() {
        let prices = vec![1, 5];
        assert_eq!(max_profit(prices), 4);
    }

    #[test]
    fn test_two_prices_no_profit() {
        let prices = vec![5, 1];
        assert_eq!(max_profit(prices), 0);
    }

    #[test]
    fn test_increasing_prices() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(max_profit(prices), 4);
    }
}
