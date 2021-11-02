// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut valley = prices[0];
    let mut  peak = prices[0];
    let mut  maxprofit = 0;
    while i < prices.len() - 1 {
        while i < prices.len() - 1 && prices[i] >= prices[i + 1] {
            i += 1;
        }
        valley = prices[i];
        while i < prices.len() - 1 && prices[i] <= prices[i + 1] {
            i += 1;
        }
        peak = prices[i];
        maxprofit += peak - valley;
    }
    maxprofit
}


#[cfg(test)]
mod tests {
    use super::max_profit;
    #[test]
    fn test_max_profit() {
        let actual = max_profit(vec![7, 1, 3, 5, 3, 6, 4]);
        let expected: i32 = 7;
        assert_eq!(actual, expected);
    }
}
