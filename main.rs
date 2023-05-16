#[derive(Debug)]
struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(val: T) -> Self {
        ListNode { val, next: None }
    }
}

#[derive(Debug)]
struct Queue<T> {
    head: Option<Box<ListNode<T>>>,
    tail: *mut ListNode<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn enqueue(&mut self, val: T) {
        let new_node = Box::new(ListNode::new(val));
