fn indexof(items: &[u32], item: u32) -> Option<usize> {
    for i in 0..items.len() {
	if items[i] == item {
            return Some(i);
	}
    }
    //
    return None;
}

const LIMIT : usize = 3;

#[rmc::proof]
pub fn test_01() {
    // Create arbitrary array
    let xs : [u32; LIMIT] = rmc::nondet();
    // Create arbitrary element
    let x : u32 = rmc::nondet();
    // Create arbitrary (valid) length
    let len : usize = rmc::nondet();
    rmc::assume(len <= LIMIT);
    // Ensure element not in array below len
    for i in 0..len {
	rmc::assume(xs[i] != x);
    }  
    // Check
    assert!(indexof(&xs[..len],x) == None);
} 

#[rmc::proof]
pub fn test_02() {
    // Create arbitrary array
    let xs : [u32; LIMIT] = rmc::nondet();
    // Create arbitrary element
    let x : u32 = rmc::nondet();
    // Create arbitrary (valid) length
    let len : usize = rmc::nondet();
    rmc::assume(len <= LIMIT);
    // Create arbitrary index within array
    let i : usize = rmc::nondet();
    rmc::assume(i < len);
    // Ensure known value at index i
    rmc::assume(xs[i] == x);    
    // Ensure nothing in array below i matches
    for j in 0..i {
	rmc::assume(xs[j] != x);
    }
    // Check find correct one
    assert!(indexof(&xs[..len],x) == Some(i));
} 

#[cfg(rmc)]
#[no_mangle]
pub fn main() {
    test_01();
    test_02();    
}
