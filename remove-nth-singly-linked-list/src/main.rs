#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            next: next,
            val,
        }
    }
}

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut len = 0;
    {
        let mut temp2_head = head.as_ref();
        while temp2_head.is_some() {
            temp2_head = temp2_head.unwrap().next.as_ref();
            len += 1;
        }
    }
    let nth = len - n;
    if nth == 0 {
        head.unwrap().next
    } else {
        {
            let mut it = 1;
            let mut new_head = head.as_mut();
            while it < nth && new_head.is_some() {
                new_head = new_head.unwrap().next.as_mut();
                it += 1;
            }
            new_head.as_mut().unwrap().next = new_head.as_mut().unwrap().next.as_mut().unwrap().next.take();
        }

        head
    }
}

fn main() {
    let head_1 = ListNode::new(1, None);
    let head_2 = ListNode::new(1, Some(Box::new(ListNode::new(2, None))));
    let head_3 = ListNode::new(1,
                               Some(Box::new(ListNode::new(2, Some(Box::new(ListNode::new(3,
                                                                                          Some(Box::new(ListNode::new(4,
                                                                                                                      Some(Box::new(ListNode::new(5,
                                                                                                                                                  None))),
                                                                                          ))),
                               ))),
                               ))));

    println!("{:?}", remove_nth_from_end(Some(Box::new(head_1)), 1));
    println!("{:?}", remove_nth_from_end(Some(Box::new(head_2.clone())), 2));
    println!("{:?}", remove_nth_from_end(Some(Box::new(head_2.clone())), 1));
    println!("{:?}", remove_nth_from_end(Some(Box::new(head_3.clone())), 3));
    println!("{:?}", remove_nth_from_end(Some(Box::new(head_3.clone())), 5));
}
