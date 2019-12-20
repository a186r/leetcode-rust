/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order and each of their nodes
 * contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 *
 * Example:
 *
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 *
 */
pub struct Solution{}
use super::util::linked_list::{to_list, ListNode};
use std::intrinsics::truncf32;

impl Solution{
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;
        let (mut l1_end, mut l2_end, mut overflow) = (false, false, false);
        loop {
            let lhs = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                },
                None => {
                    l1_end = true;
                    0
                },
            };
            let rhs = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                },
                None => {
                    l2_end = true;
                    0
                }
            };
//            如果l1_end, l2_end结束并且咩有溢出，则返回结果
            if l1_end && l2_end && !overflow{
                break dummy_head.unwrap().next;
            }

            let sum = lhs + rhs + if overflow {1} else { 0 };

            let sum = if sum >= 10 {
                overflow = true;
                sum - 10
            }else {
                overflow = false;
                sum
            };
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
}