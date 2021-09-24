fn indexof(items: &[u32], item: u32) -> usize {
    for i in 0..items.len() {
	if items[i] == item {
            return i;
	}
    }
    //
    return usize::MAX;
}

fn __nondet<T>() -> T {
    unimplemented!()
}
fn __VERIFIER_assume(cond: bool) {
    unimplemented!()
}

/**
 * Ensure that a given item is not in the give array.
 */
#[cfg(rmc)]
#[no_mangle]
fn disjoint(items : &[u32], item: u32) {
    for ith in items {
	__VERIFIER_assume(*ith != item);
    }        
}

#[cfg(rmc)]
#[no_mangle]
pub fn test_01() {
    // Create arbitrary array of size 3
    let xs : [u32; 3] = __nondet();
    // Ensure element not in array
    for x in xs {
	__VERIFIER_assume(x != 0);
    }        
    //
    assert!(indexof(&xs,0) == usize::MAX);
} 

#[cfg(rmc)]
#[no_mangle]
pub fn test_02() {
    // Create arbitrary array of size 3
    let xs : [u32; 2] = __nondet();
    // Create arbitrary index within array
    let i : usize = __nondet();
    __VERIFIER_assume(i < xs.len());
    // Ensure known value at that point
    __VERIFIER_assume(xs[i] == 0);    
    // Ensure nothing in array below i matches
    for j in 0..2 {
	if j != i {
	    __VERIFIER_assume(xs[j] != 0);
	}
    }
    //
    assert!(indexof(&xs,0) == i);
} 

#[cfg(rmc)]
#[no_mangle]
pub fn main() {
    //test_01();
    test_02();    
}
