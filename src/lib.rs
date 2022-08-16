mod listnode;
mod list;
mod queue;
mod stack;
pub use list::*;
pub use queue::*;
pub use stack::*;

use listnode::*;
pub trait ListOperation<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn iter(&self) -> ListIter<'_, T>;
    fn iter_mut(&mut self) -> ListIterMut<'_, T>;
    fn isempty(&self) -> bool;
    fn len(&self) -> usize;
}

pub struct ListIter<'a, T> {
    node: Option<&'a ListNode<T>>
}

pub struct ListIterMut<'a, T> {
    node: Option<&'a mut ListNode<T>>
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
