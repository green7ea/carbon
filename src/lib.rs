mod discover;

use discover::{discover, HostIterator};
use std::{ffi::CString, os::raw::c_char};

#[no_mangle]
pub extern "C" fn create_host_iter() -> *mut HostIterator
{
    // TODO fix unwrap
    Box::into_raw(Box::new(discover().ok().unwrap()))
}

#[no_mangle]
pub extern "C" fn free_host_iter(iter_ptr: *mut HostIterator)
{
    if !iter_ptr.is_null()
    {
        unsafe { Box::from_raw(iter_ptr) };
    }
}

#[no_mangle]
pub extern "C" fn next_host(iter_ptr: *mut HostIterator) -> *mut c_char
{
    if iter_ptr.is_null()
    {
        return 0 as *mut c_char;
    }

    unsafe {
        (*iter_ptr).next().map_or(0 as *mut c_char, |x| {
            CString::new(x.to_string())
                .map_or(0 as *mut c_char, CString::into_raw)
        })
    }
}

#[no_mangle]
pub extern "C" fn free_host(hostname_ptr: *mut c_char)
{
    // We retake possession of the pointer so it gets cleaned up.
    unsafe {
        CString::from_raw(hostname_ptr);
    }
}
