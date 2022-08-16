use std::ptr;
use crate::listnode::ListNode;
use crate::{ListOperation, ListIter, ListIterMut};

pub struct List<T> {
    head: *mut ListNode<T>,
    tail: *mut ListNode<T>,
    len: usize,
}

impl <T> List<T> {
    pub fn new() -> Self {
	Self {
	    head: ptr::null_mut(),
	    tail: ptr::null_mut(),
	    len: 0
	}
    }

    pub fn front(&self) -> Option<&T> {
	unsafe {
	    self.head.as_ref().map(|head| {
		&head.value
	    })
	}
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
	unsafe {
	    self.head.as_mut().map(|head| {
		&mut head.value
	    })
	}
    }


}

impl <T> ListOperation<T> for List<T> {
    fn push(&mut self, value: T) {
	// let mut newnode = ListNode::new(value);

	unsafe {
	    let newnode = Box::into_raw(Box::new(ListNode::new(value)));
	    if self.head.is_null() {
		self.tail = newnode;
		self.head = self.tail;
	    } else {
		// ATTENTION where is prev, the same as queue
		(*newnode).prev = self.tail;
		(*(self.tail)).next = newnode;
		self.tail = (*(self.tail)).next;
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
		let tail = Box::from_raw(self.tail);
		self.tail = tail.prev;

		if self.tail.is_null() {
		    self.head = ptr::null_mut();
		}

		Some(tail.value)
	    }
	}


	// unsafe {
	//     if self.tail.is_null() {
	// 	None
	//     } else {
	// 	let tail = self.tail;
	// 	self.tail = (*tail).prev;

	// 	if self.tail.is_null() {
	// 	    self.head = ptr::null_mut();
	// 	}

	// 	// this is the same as return self.tail.value,
	// 	// which is behind a shared reference, that I cannot move out
	//      // the main reason is self.tail not move to tail
	// 	Some((*tail).value)
	//     }
	// }
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

impl <T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {
	    
	}
    }
}