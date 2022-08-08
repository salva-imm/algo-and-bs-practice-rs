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
fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut reversed: Option<Box<ListNode>> = None;
    {
        let mut temp_head = head.as_mut();
        while temp_head.is_some() {
            let x = temp_head.as_mut().unwrap().clone();
            let temp = reversed.clone();
            reversed = Some(x);
            reversed.as_mut().unwrap().next = temp;
            temp_head = temp_head.unwrap().next.as_mut();
        }
    }
    reversed
}
fn main() {
    let head_1 = ListNode::new(1, None);
    let head_2 = ListNode::new(1, Some(Box::new(ListNode::new(2, None))));
    let head_3 = ListNode::new(
        1,
        Some(Box::new(ListNode::new(
            2,
            Some(Box::new(ListNode::new(
                3,
                Some(Box::new(ListNode::new(
                    4,
                    Some(Box::new(ListNode::new(5, None))),
                ))),
            ))),
        ))),
    );
    println!("{:?}", reverse_list(Some(Box::new(head_1))));
    println!("{:?}", reverse_list(Some(Box::new(head_2))));
    println!("{:?}", reverse_list(Some(Box::new(head_3))));
    println!("{:?}", reverse_list(None));
}
