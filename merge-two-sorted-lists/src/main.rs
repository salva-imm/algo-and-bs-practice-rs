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

fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = list1.clone();
    let mut list2 = list2.clone();
    let mut head: Option<Box<ListNode>> = None;
    if list1.is_some() && list2.is_some() {
        if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
            head = Some(Box::new(ListNode::new(list1.as_mut().unwrap().val, None)));
            list1 = list1.unwrap().next.take();
        } else {
            head = Some(Box::new(ListNode::new(list2.as_mut().unwrap().val, None)));
            list2 = list2.unwrap().next.take();
        }
        {
            let mut temp_head = head.as_mut();
            while list1.is_some() && list2.is_some() {
                if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                    temp_head.as_mut().unwrap().next =
                        Some(Box::new(ListNode::new(list1.as_mut().unwrap().val, None)));
                    temp_head = temp_head.unwrap().next.as_mut();
                    list1 = list1.unwrap().next.take();
                } else {
                    temp_head.as_mut().unwrap().next =
                        Some(Box::new(ListNode::new(list2.as_mut().unwrap().val, None)));
                    temp_head = temp_head.unwrap().next.as_mut();
                    list2 = list2.unwrap().next.take();
                }
            }
            while list1.is_some() {
                temp_head.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(list1.as_mut().unwrap().val, None)));
                temp_head = temp_head.unwrap().next.as_mut();
                list1 = list1.unwrap().next.take();
            }
            while list2.is_some() {
                temp_head.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(list2.as_mut().unwrap().val, None)));
                temp_head = temp_head.unwrap().next.as_mut();
                list2 = list2.unwrap().next.take();
            }
        }
    } else if list1.is_some() && list2.is_none() {
        head = list1
    } else if list2.is_some() && list1.is_none() {
        head = list2
    }
    head
}

fn main() {
    let head_1 = ListNode::new(
        1,
        Some(Box::new(ListNode::new(
            3,
            Some(Box::new(ListNode::new(4, None))),
        ))),
    );
    let head_2 = ListNode::new(
        1,
        Some(Box::new(ListNode::new(
            2,
            Some(Box::new(ListNode::new(4, None))),
        ))),
    );
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
    println!(
        "{:?}",
        merge_two_lists(Some(Box::new(head_1)), Some(Box::new(head_2)))
    );
}
