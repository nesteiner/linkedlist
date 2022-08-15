use std::ptr;
use crate::listnode::ListNode;

pub struct List<T> {
    head: *mut ListNode<T>,
    tail: *mut ListNode<T>,
    len: usize,
}

pub struct ListIter<'a, T> {
    node: Option<&'a ListNode<T>>
}

pub struct ListIterMut<'a, T> {
    node: Option<&'a mut ListNode<T>>
}

impl <T> List<T> {
    pub fn new() -> Self {
	Self {
	    head: ptr::null_mut(),
	    tail: ptr::null_mut(),
	    len: 0
	}
    }

    pub fn pushback(&mut self, value: T) {
	// let mut newnode = ListNode::new(value);

	unsafe {
	    let newnode = Box::into_raw(Box::new(ListNode::new(value)));
	    if self.head.is_null() {
		self.tail = newnode;
		self.head = self.tail;
	    } else {
		(*(self.tail)).next = newnode;
		self.tail = (*(self.tail)).next;
	    }
	}

	self.len += 1;
    }

    pub fn pushfront(&mut self, value: T) {
	let mut newnode = Box::into_raw(Box::new(ListNode::new(value)));
	if self.head.is_null() {
	    self.tail = newnode;
	    self.head = self.tail;
	} else {
	    unsafe {
		(*(newnode)).next = self.head;
		(*(self.head)).prev = newnode;
		self.head = newnode;
	    }
	}

	self.len += 1;
    }

    pub fn popfront(&mut self) -> Option<T> {
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

    pub fn popback(&mut self) -> Option<T> {
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

    pub fn iter(&self) -> ListIter<T> {
	unsafe {
	    ListIter { 
		node: self.head.as_ref()
	    }
	}
    }

    pub fn iter_mut(&mut self) -> ListIterMut<'_, T> {
	unsafe {
	    ListIterMut {
		node: self.head.as_mut()
	    }
	}
    }
}

impl <'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
	unsafe {
            self.node.take().map(|node| {
		self.node = node.next.as_ref();
		&node.value
	    })
	}
    }
}

impl <'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
	    self.node.take().map(|node| {
		self.node = node.next.as_mut();
		&mut node.value
	    })
	}
    }
}

impl <T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.popfront() {
	    
	}
    }
}