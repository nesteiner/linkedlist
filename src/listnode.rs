use std::ptr;

pub struct ListNode<T> {
    pub value: T,
    pub prev: *mut ListNode<T>,
    pub next: *mut ListNode<T>
}

impl <T> ListNode<T> {
    pub fn new(value: T) -> Self {
	Self {
	    value,
	    prev: ptr::null_mut(),
	    next: ptr::null_mut()
	}
    }

    // pub fn insert_next(&mut self, mut node: ListNode<T>) {
    // 	unsafe {
    // 	    node.prev = self;
    // 	    *(self.next) = node;
    // 	}
    // }
}