
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

// https://leetcode.com/problems/maximal-score-after-applying-k-operations/description/
impl Solution {
    pub fn max_k_elements(nums: Vec<i32>, k: i32) -> i64 {
        let mut ans: i64 = 0;

        // Create a max-heap to store the elements (represented using BinaryHeap as a max-heap)
        let mut pq: BinaryHeap<i32> = nums.into_iter().collect();

        let mut k = k;
        while k > 0 {
            // Extract the maximum element
            let max_element = pq.pop().unwrap();

            // Add the maximum element to the answer
            ans += max_element as i64;

            // Push its one-third value (rounded up) back into the heap
            let reduced_value = (max_element as f64 / 3.0).ceil() as i32;
            pq.push(reduced_value);

            // Decrement k to count the number of operations
            k -= 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_steps() {
        // assert_eq!(Solution::max_k_elements(vec![10, 15, 20, 24, 30], 3), 16);
        // assert_eq!(Solution::max_k_elements(vec![9, 12, 5], 4), 9);
        assert_eq!(Solution::max_k_elements(vec![10,10,10,10,10], 5), 50);
        assert_eq!(Solution::max_k_elements(vec![1,10,3,3,3], 3), 17);
    }
}
