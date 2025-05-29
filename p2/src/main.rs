//Solution passes all tests with 0ms runtime (meets or beats 100% of competitors) and 2.27MB memory
//(meets or beats 59.67% of competitors)

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// LeetCode always uses a dummy Solution struct to evaluate code
struct Solution;

fn new_node(val: i32) -> Box<ListNode> {
    Box::new(if val < 10 {
        ListNode::new(val)
    } else {
        ListNode {
            val: 1,
            next: Some(new_node(val / 10)),
        }
    })
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::add_two_numbers_and_carry(l1, l2, false)
    }

    pub fn add_two_numbers_and_carry(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: bool,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(l1), Some(l2)) => {
                let val = l1.val + l2.val + if carry { 1 } else { 0 };
                Some(Box::new(ListNode {
                    val: val % 10,
                    next: Solution::add_two_numbers_and_carry(l1.next, l2.next, val > 9),
                }))
            }
            (Some(l1), None) | (None, Some(l1)) => {
                let val = l1.val + if carry { 1 } else { 0 };
                Some(Box::new(ListNode {
                    val: val % 10,
                    next: Solution::add_two_numbers_and_carry(l1.next, None, val > 9),
                }))
            }
            (None, None) => {
                if carry {
                    Some(Box::new(ListNode::new(1)))
                } else {
                    None
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
