
#[no_mangle]
pub unsafe extern fn __aeabi_memcpy(dest: *mut u8, src: *mut u8, n: isize) -> *mut u8 {
    let mut i =  0;
    while i < n {
        *dest.offset(i) = *src.offset(i);
        i += 1;
    }
    return dest;
}

#[no_mangle]
pub unsafe extern fn __aeabi_memset(dest: *mut u8, val: i32, n: isize) -> *mut u8 {
    for i in 0..n {
        *dest.offset(i) = val as u8;
    }
    return dest;
}

#[no_mangle]
pub unsafe extern fn __aeabi_memclr(dest: *mut u8, n: isize) -> *mut u8 {
    let mut i =  0;
    while i < n {
        *dest.offset(i) = 0u8;
        i += 1;
    }
    return dest;
}

extern {
    pub static _etext : u8;
    pub static _data : u8;
    pub static _edata : u8;
    pub static _bstart : u8;
    pub static _bend : u8;
}

#[macro_export]
macro_rules! addr_of {
    ($id:expr) => (
        (&$id as *const u8) as *mut u8
    )
}

pub unsafe fn init_mem() {
    // zero .bss section
    __aeabi_memclr(addr_of!(_bstart), addr_of!(_bend) as isize - addr_of!(_bstart) as isize);
    // copy .data section
    __aeabi_memcpy(addr_of!(_data), addr_of!(_etext), addr_of!(_edata) as isize - addr_of!(_data) as isize);
    
}

