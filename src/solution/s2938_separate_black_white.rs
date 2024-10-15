
pub struct Solution {}

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut total_swaps: i64 = 0;
        let mut black_ball_count: i32 = 0;

        // Iterate through each ball (character) in the string
        for ch in s.chars() {
            if ch == '0' {
                // If it's a white ball (0), add the number of black balls encountered so far
                total_swaps += black_ball_count as i64;
            } else {
                // Increment black ball count (for black balls '1')
                black_ball_count += 1;
            }
        }

        total_swaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_steps() {
        assert_eq!(Solution::minimum_steps("010010".to_string()), 4);
        assert_eq!(Solution::minimum_steps("101".to_string()), 1);
        assert_eq!(Solution::minimum_steps("100".to_string()), 2);
        assert_eq!(Solution::minimum_steps("0111".to_string()), 0);
    }
}
