//Solution passes all tests with 1ms runtime (meets or beats 100% of competitors) and 2.37MB memory
//(meets or beats 32.72% of competitors)

use std::collections::HashMap;

// LeetCode always uses a dummy Solution struct to evaluate code
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut index: isize = 0;
        let mut longest: i8 = 0;
        let mut indices: HashMap<char, isize> = HashMap::new();

        for (i, character) in s.chars().enumerate() {
            if let Some(w) = indices.get(&character) {
                if *w >= index {
                    index = *w + 1;
                }
            }
            longest = longest.max((i - index as usize + 1) as i8);
            indices.insert(character, i);
        }
        longest as i32
    }
}

fn main() {
    println!("Hello, world!");
}
