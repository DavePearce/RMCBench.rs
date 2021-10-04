use std::alloc::{alloc, dealloc, Layout};
use std::mem;

struct Array<T> {
    ptr: * mut T,
    len: usize,
}

impl<T> Array<T> {
    fn new(len: usize) -> Array<T> {
	// FIXME: this is obviously broken as we haven't constructed
	// the values properly?
	let ptr = unsafe {
	    let layout = Layout::array::<T>(len);	    
	    alloc(layout.unwrap())
	};
	Array {
	    ptr: ptr as * mut T,
	    len: len
	}
    }
    fn get_mut(&self, index: usize) -> &mut T {
	if index >= self.len { panic!(); }
	unsafe {
	    let ptr = self.ptr.offset(index as isize);
	    &mut *ptr
	}
    }
    // fn fill(&mut self, value: T) {
    // 	for i in 0 .. self.len {
    // 	    * self.get_mut(i) = value;
    // 	}
    // }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
	unsafe {
	    for i in 0..self.len {
		std::ptr::drop_in_place(self.ptr.offset(i as isize));
	    }
	}
    }
}
