/*
Author: Matthew Tindley

Summary:
    The DynamicArray type is used to create a block of n bytes memory that can be 
    referenced and cast freely
*/

use std::alloc::{alloc, dealloc, Layout};


pub struct DynamicArray {
    // The base memory pointer
    mem: *mut u8,

    // The shape of the memory
    // This is given by a Layout object which holds the raw size and alignment
    shape: Layout
}


/* Implementation */
impl Dynamicarray {

    // Creates a new DynamicArray type
    // This function allocates raw memory
    fn new(layout: Layout) -> Self {
        unsafe {
            DynamicArray {
                mem: alloc(layout),
                shape: layout
            }
        }
    }

    // Takes memory from the offset and casts it into T
    fn get<T>(&self, offset: usize) -> &T {
        unsafe {
            return &(*self.mem.offset(offset as isize).cast::<T>());
        }
    }
    // Same as get but mutable
    fn get_mut<T>(&self, offset: usize) -> &mut T {
        unsafe {
            return &mut (*self.mem.offset(offset as isize).cast::<T>());
        }
    }
}

// Auto free memory when the type is dropped
impl Drop for DynamicArray {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.mem, self.shape);
        }
    }
}