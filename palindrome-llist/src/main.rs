use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { next: next, val }
    }
}
#[inline]
fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut head = head;
    let mut reverse_head: Vec<i32> = vec![0; 100000];
    let mut palindrome_state = true;
    let mut len = 0;
    while head.is_some() {
        let data = head.as_ref().unwrap().val;
        reverse_head[len] = data;
        head = head.unwrap().next.take();
        len += 1;
    }
    (0..len / 2)
        .map(|i| {
            if reverse_head[i] != reverse_head[len - i - 1] {
                palindrome_state = false;
            }
        })
        .count();
    palindrome_state
}

fn main() {
    let head_1 = ListNode::new(1, None);
    let head_2 = ListNode::new(1, Some(Box::new(ListNode::new(1, None))));
    let head_3 = ListNode::new(1, Some(Box::new(ListNode::new(1, None))));
    println!("{:?}", is_palindrome(Some(Box::new(head_3))));
    // println!("{:?}", reverse_list(Some(Box::new(head_2))));
    // println!("{:?}", reverse_list(Some(Box::new(head_3))));
    // println!("{:?}", reverse_list(None));
}
