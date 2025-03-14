#![no_std]
#![no_main]
#![cfg(target_arch = "wasm32")]

extern crate alloc;

use alloc::vec::Vec;
use core::mem;
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};
use unicode_shaper::*;

// SAFETY: This application is single threaded, so using AssumeSingleThreaded is allowed.
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

mod wasm_specific {
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        loop {}
    }
}

// Declare the external JavaScript function
extern "C" {
    fn setUnicodeArray(ptr: *const u16, size: usize);
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn processText(input_ptr: *const u16, len: usize, options: u32) {
    // Convert the input pointer and length to a slice
    let input_slice = core::slice::from_raw_parts(input_ptr, len);
    // Modify the input data
    let result_vec = shape_unicode(input_slice, &options);
    // Call setUnicodeArray to pass the resultant data to JavaScript
    setUnicodeArray(result_vec.as_ptr(), result_vec.len());
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn allocUnicodeArray(size: usize) -> *mut u16 {
    // Allocate memory
    let mut buffer: Vec<u16> = Vec::with_capacity(size);
    buffer.capacity();
    // Ensure capacity matches size to avoid resizing
    buffer.set_len(size);
    // Get a raw pointer to the allocated memory
    let ptr = buffer.as_mut_ptr();
    // Prevent the buffer from being deallocated when it goes out of scope
    mem::forget(buffer);

    ptr
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u16, size: usize) {
    // Convert the pointer to a slice and then drop it
    let _ = core::slice::from_raw_parts_mut(ptr, size);

    // Deallocate the memory
    alloc::alloc::dealloc(ptr as *mut u8, alloc::alloc::Layout::array::<u16>(size).unwrap());
}

#[no_mangle]
pub extern "C" fn isRTL(input: u16) -> bool {
    is_rtl(&input)
}

#[no_mangle]
pub extern "C" fn isCJK(input: u16) -> bool {
    is_cjk(&input)
}
