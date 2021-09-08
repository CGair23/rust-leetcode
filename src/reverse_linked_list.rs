use std::boxed::Box;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre: Option<Box<ListNode>> = None;
        let mut curr = head;

        while let Some(mut boxed_node) = curr {
            let mut next = boxed_node.next.take();
            boxed_node.next = pre;
            pre = Some(boxed_node);
            curr = next;
        }

        pre
    }
}

#[inline]
pub fn test_reverse_list() {
    let head = 
    Some(Box::new(
        ListNode{
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 2,
                    next: Some(Box::new(
                        ListNode {
                            val: 3,
                            next: Some(Box::new(
                                ListNode {
                                    val: 4,
                                    next: None
                                }
                            ))
                        }
                    ))
                }
            ))
        }
    ));
    Solution::reverse_list(head);
}
