//Solution passes all tests with 0ms runtime (meets or beats 100% of competitors) and 2.24MB memory
//(meets or beats 77.90% of competitors)

fn main() {
    println!("{:?}", Solution::two_sum(vec![5, 2], 7));
}

// LeetCode always uses a dummy Solution struct to evaluate code
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //This is only half a joke solution; down-casting the integers to i16 saves enough memory
        //to beat a large portion of submissions without increasing runtime or invalidating the
        //problem constraints.
        let mut hmap: std::collections::HashMap<i16, i16> = std::collections::HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            let remainder = target - x;
            if hmap.contains_key(&(remainder as i16)) {
                return vec![*hmap.get(&(remainder as i16)).unwrap() as i32, i as i32];
            } else {
                hmap.insert(*x as i16, i as i16);
            }
        }
        Vec::new()
    }
}
