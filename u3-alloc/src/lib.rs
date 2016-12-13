//! Allocator for jet code for u3
//!
//! Jets in u3 can't use regular memory allocation. They must use this custom allocator to allocate
//! value from inside the u3 loom instead.

#![feature(allocator)]
#![allocator]
#![no_std]

extern "C" {
    fn u3a_malloc(len_i: usize) -> *mut u8;
    fn u3a_free(tox_v: *mut u8);
    fn u3a_realloc(lag_v: *mut u8, len_i: usize) -> *mut u8;
}

#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe { u3a_malloc(size) }
}

#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe { u3a_free(ptr) }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate(ptr: *mut u8,
                                    _old_size: usize,
                                    size: usize,
                                    _align: usize)
                                    -> *mut u8 {
    unsafe { u3a_realloc(ptr, size) }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(_ptr: *mut u8,
                                            old_size: usize,
                                            _size: usize,
                                            _align: usize)
                                            -> usize {
    old_size
}

#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
