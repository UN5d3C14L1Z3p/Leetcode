// Easy
//
// Given an array of integers nums and an integer target, return indices of the two numbers such
// that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same
// element twice.
// You can return the answer in any order.
//
// Example 1:
//
// Input: nums = [2, 7, 11, 15], target = 9
// Output: [0, 1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
// Example 2:
//
// Input: nums = [3, 2, 4], target = 6
// Output: [1, 2]
//
// Example 3:
//
// Input: nums = [3, 3], target = 6
// Output: [0, 1]
//
// Constraints:
//
// * 2 <= nums.length <= 10^4
// * -10^9 <= nums[1] <= 10^9
// * -10^9 <= target <= 10^9
// * Only one valid answer exists.
//
// Follow-up: Can you come up with an algorithm that is less than O(n^2) time complexity?
//

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: std::collections::HashMap<i32, i32> = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            let c = target - v;
            if m.contains_key(&c) {
                if let Some(v) = m.get(&c) {
                    return [*v, i.try_into().unwrap()].to_vec();
                }
            }
            m.insert(*v, i.try_into().unwrap());
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::two_sum([2, 7, 11, 15].to_vec(), 9),
            [0, 1].to_vec()
        );

        assert_eq!(Solution::two_sum([3, 2, 4].to_vec(), 6), [1, 2].to_vec());
        assert_eq!(Solution::two_sum([3, 3].to_vec(), 6), [0, 1].to_vec());
    }
}
