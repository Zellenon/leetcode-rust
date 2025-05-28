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
        if !carry && l1.is_none() && l2.is_none() {
            None
        } else {
            let val = l1.map(|w| w.val).unwrap_or(0)
                + l2.map(|w| w.val).unwrap_or(0)
                + (if carry { 1 } else { 0 });
            Some(Box::new(ListNode {
                val: val % 10,
                next: Solution::add_two_numbers_and_carry(
                    l1.map(|w| w.next).flatten(),
                    l2.map(|w| w.next).flatten(),
                    val > 9,
                ),
            }))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
