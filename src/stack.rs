use std::ptr;
use crate::listnode::ListNode;
use crate::{ListOperation, ListIter, ListIterMut};

pub struct Stack<T> {
    head: *mut ListNode<T>,
    tail: *mut ListNode<T>,
    len: usize,
}


impl <T> Stack<T> {
    pub fn new() -> Self {
	Self {
	    head: ptr::null_mut(),
	    tail: ptr::null_mut(),
	    len: 0
	}
    }

    pub fn top(&self) -> Option<&T> {
	unsafe {
	    self.head.as_ref().map(|head| {
		&head.value
	    })
	}
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
	unsafe {
	    self.head.as_mut().map(|head| {
		&mut head.value
	    })
	}
    }


}

impl <T> ListOperation<T> for Stack<T> {
    fn push(&mut self, value: T) {
	// let mut newnode = ListNode::new(value);

	unsafe {
	    let newnode = Box::into_raw(Box::new(ListNode::new(value)));
	    if self.head.is_null() {
		self.tail = newnode;
		self.head = self.tail;
	    } else {
		(*(self.head)).prev = newnode;
		(*newnode).next = self.head;
		self.head = newnode;
	    }
	}

	self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
	unsafe {
	    if self.head.is_null() {
		None
	    } else {
		self.len -= 1;
		let head = Box::from_raw(self.head);
		self.head = head.next;

		if self.head.is_null() {
		    self.tail = ptr::null_mut();
		}

		Some(head.value)
	    }
	}

    }

    fn iter(&self) -> ListIter<T> {
	unsafe {
	    ListIter { 
		node: self.head.as_ref()
	    }
	}
    }

    fn iter_mut(&mut self) -> ListIterMut<'_, T> {
	unsafe {
	    ListIterMut {
		node: self.head.as_mut()
	    }
	}
    }

    fn isempty(&self) -> bool {
        return self.len() == 0;
    }

    fn len(&self) -> usize {
        return self.len;
    }

}


impl <T> Drop for Stack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {
	    
	}
    }
}