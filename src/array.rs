use std::alloc::{alloc, dealloc, Layout};
use std::mem;

struct Array<T> {
    ptr: * mut T,
    len: usize,
}

impl<T> Array<T>
where T : Copy {
    fn new(elem: T, len: usize) -> Array<T> {
	let _ptr = unsafe {
	    let layout = Layout::array::<T>(len);	    
	    alloc(layout.unwrap())
	};
        let ptr = _ptr as * mut T;
        unsafe {
            for i in 0..len {
                ptr.offset(i as isize).write(elem);
            }
        }
	Array {
	    ptr: ptr,
	    len: len
	}
    }
    
    /// Return the length of this fixed array.
    fn len(&self) -> usize {
        self.len
    }

    /// Return a mutable reference to an element of this array.
    fn get(&self, index: usize) -> &T {
	if index >= self.len { panic!(); }
	unsafe {
	    let ptr = self.ptr.offset(index as isize);
	    &*ptr
	}
    }
    
    /// Return a mutable reference to an element of this array.
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

// ======================================================
// Tests
// ======================================================

#[test]
fn test_01() {
    let arr = Array::<u32>::new(0,0);
    assert_eq!(arr.len(),0);
}

#[test]
fn test_02() {
    let arr = Array::<u32>::new(0,1);
    assert_eq!(arr.len(),1);
    assert_eq!(*arr.get(0),0);
}


