pub struct Buffer<T,const N:usize> {
    r_ptr : usize,
    w_ptr : usize,
    items: [T;N]
}

impl<T, const N:usize> Buffer<T,N> {
    //
    pub fn invariant(&self) -> bool {
	self.r_ptr < N && self.w_ptr < N
    }
    
    pub fn len(&self) -> usize {
	if self.w_ptr > self.r_ptr {
	    // Normal case
	    self.w_ptr - self.r_ptr
	} else {
	    // Wrap around case
	    (self.w_ptr + N) - self.r_ptr
	}
    }
}

impl<T, const N:usize> Buffer<T,N>
where T:Copy {
    //
    pub fn new(value : T) -> Self {
	Buffer{ r_ptr:0, w_ptr:0, items: [value;N] }
    }
}

fn __nondet<T>() -> T { unimplemented!() }
fn __VERIFIER_assume(cond: bool) { unimplemented!() }

pub fn main() {
    let buf : Buffer<u32,3> = __nondet();
    __VERIFIER_assume(buf.invariant());
    assert!(buf.len() <= 3);
}


// #[cfg(rmc)]
// #[no_mangle]
// pub fn test_01() {
//     let xs : [u32; 2] = __nondet();
//     assert!(indexof(&xs,0) == usize::MAX);
// }

// #[cfg(rmc)]
// #[no_mangle]
// pub fn main() {
//     test_01()
// }

// macro_rules! forall {
//     ([ $($elem:tt),* ]) => ({
// 	$( assert!($elem >= 0); )*	
//     })
// }
// pub fn main() {
//     //
//     let x = 0;
//     forall!([x,1,2]);
// }
