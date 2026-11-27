//! Tests for the stabilized raw slice getter methods.

use std::ptr;
use std::ptr::NonNull;

#[test]
fn test_mut_slice_getters() {
    let mut arr = [1, 2, 3];
    let ptr: *mut [i32] = arr.as_mut_ptr();
    
    // Test as_mut_ptr
    assert_eq!(ptr.as_mut_ptr(), arr.as_mut_ptr());
    
    // Test get_unchecked_mut
    unsafe {
        let elem_ptr = ptr.get_unchecked_mut(1);
        *elem_ptr = 4;
        assert_eq!(arr[1], 4);
        
        // Test with range
        let range_ptr = ptr.get_unchecked_mut(1..=2);
        assert_eq!(*range_ptr, &mut [4, 3] as *mut [i32]);
    }
}

#[test]
fn test_const_slice_getters() {
    let arr = [1, 2, 3];
    let ptr: *const [i32] = arr.as_ptr();
    
    // Test as_ptr
    assert_eq!(ptr.as_ptr(), arr.as_ptr());
    
    // Test get_unchecked
    unsafe {
        let elem_ptr = ptr.get_unchecked(1);
        assert_eq!(*elem_ptr, 2);
        
        // Test with range
        let range_ptr = ptr.get_unchecked(1..=2);
        assert_eq!(*range_ptr, &[2, 3] as *const [i32]);
    }
}

#[test]
fn test_non_null_slice_getters() {
    let mut arr = [1, 2, 3];
    let ptr = NonNull::new(arr.as_mut_ptr()).unwrap();
    
    // Test as_non_null_ptr
    assert_eq!(ptr.as_non_null_ptr(), NonNull::new(arr.as_ptr()).unwrap());
    
    // Test as_mut_ptr
    assert_eq!(ptr.as_mut_ptr(), arr.as_mut_ptr());
    
    // Test get_unchecked_mut
    unsafe {
        let elem_ptr = ptr.get_unchecked_mut(1);
        *elem_ptr = 4;
        assert_eq!(arr[1], 4);
        
        // Test with range
        let range_ptr = ptr.get_unchecked_mut(1..=2);
        assert_eq!(*range_ptr, NonNull::new(&mut [4, 3] as *mut [i32]).unwrap());
    }
}
