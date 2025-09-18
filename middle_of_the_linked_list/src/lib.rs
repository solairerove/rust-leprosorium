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

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut fast, mut slow) = (&head, &head);
    while let Some(ListNode {
        next: Some(node), ..
    }) = fast.as_deref()
    {
        slow = &slow.as_ref().unwrap().next;
        fast = &node.next;
    }

    slow.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vals.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }

    #[test]
    fn test_example_1() {
        let head = create_list(vec![1, 2, 3, 4, 5]);
        let result = middle_node(head);
        assert_eq!(list_to_vec(result), vec![3, 4, 5]);
    }

    #[test]
    fn test_example_2() {
        let head = create_list(vec![1, 2, 3, 4, 5, 6]);
        let result = middle_node(head);
        assert_eq!(list_to_vec(result), vec![4, 5, 6]);
    }

    #[test]
    fn test_single_node() {
        let head = create_list(vec![1]);
        let result = middle_node(head);
        assert_eq!(list_to_vec(result), vec![1]);
    }

    #[test]
    fn test_two_nodes() {
        let head = create_list(vec![1, 2]);
        let result = middle_node(head);
        assert_eq!(list_to_vec(result), vec![2]);
    }

    #[test]
    fn test_three_nodes() {
        let head = create_list(vec![1, 2, 3]);
        let result = middle_node(head);
        assert_eq!(list_to_vec(result), vec![2, 3]);
    }

    #[test]
    fn test_four_nodes() {
        let head = create_list(vec![1, 2, 3, 4]);
        let result = middle_node(head);
        assert_eq!(list_to_vec(result), vec![3, 4]);
    }
}
