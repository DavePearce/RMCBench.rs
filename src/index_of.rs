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

const LIMIT : usize = 5;

#[cfg(rmc)]
#[no_mangle]
pub fn test_01() {
    // Create arbitrary array
    let xs : [u32; LIMIT] = __nondet();
    // Create arbitrary element
    let x : u32 = __nondet();
    // Create arbitrary (valid) length
    let len : usize = __nondet();
    __VERIFIER_assume(len <= LIMIT);
    // Ensure element not in array below len
    for i in 0..len {
	__VERIFIER_assume(xs[i] != x);
    }  
    // Check
    assert!(indexof(&xs[..len],x) == usize::MAX);
} 

#[cfg(rmc)]
#[no_mangle]
pub fn test_02() {
    // Create arbitrary array
    let xs : [u32; LIMIT] = __nondet();
    // Create arbitrary element
    let x : u32 = __nondet();
    // Create arbitrary (valid) length
    let len : usize = __nondet();
    __VERIFIER_assume(len <= LIMIT);
    // Create arbitrary index within array
    let i : usize = __nondet();
    __VERIFIER_assume(i < len);
    // Ensure known value at index i
    __VERIFIER_assume(xs[i] == x);    
    // Ensure nothing in array below i matches
    for j in 0..i {
	__VERIFIER_assume(xs[j] != x);
    }
    // Check find correct one
    assert!(indexof(&xs[..len],x) == i);
} 

#[cfg(rmc)]
#[no_mangle]
pub fn main() {
    test_01();
    test_02();    
}
