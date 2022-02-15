/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * Example:
 *
 * Input: "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 */

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: &str) -> u32 {
        let s_vec: Vec<char> = s.chars().collect();
        let max_len = 0;

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // char(&self)
    fn test_string_methods() {
        let word = "goodbye";

        let count = word.chars().count();
        assert_eq!(7, count);

        let mut word = "hello".chars();
        assert_eq!(Some('h'), word.next());
        assert_eq!(Some('e'), word.next());
        assert_eq!(Some('l'), word.next());
        assert_eq!(Some('l'), word.next());
        assert_eq!(Some('o'), word.next());
    }

    fn test_3() {

    }
}
