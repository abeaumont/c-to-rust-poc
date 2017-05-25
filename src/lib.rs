use std::mem;
use std::ffi::{CStr,CString};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct MyStruct {
    a: isize,
    b: *const c_char,
    c: MyStructPtr
}

pub type MyStructPtr = *const MyStruct;

#[no_mangle]
pub unsafe extern fn new_struct(a: isize, b: *const c_char, c: MyStructPtr) -> MyStructPtr {
    let s : Box<MyStruct> = Box::new(MyStruct{a, b, c});
    mem::transmute(s)
}

#[no_mangle]
pub unsafe extern fn use_struct(s: *mut MyStruct) {
    let mut s : Box<MyStruct> = mem::transmute(s);
    println!("Old int value: {}", s.a);
    s.a *= 2;
    println!("New int value: {}", s.a);

    let mut b1 = CStr::from_ptr(s.b).to_str().unwrap().to_string();
    println!("Old string value: {}", b1);
    b1.push_str("_add");
    s.b = b1.as_ptr() as *const c_char;
    println!("New string value: {}", b1);

    if !s.c.is_null() {
        let c : Box<MyStruct> = mem::transmute(s.c);
        println!("Other struct: {:?}\n", c);
    }
    mem::forget(s)
}

#[no_mangle]
pub unsafe extern fn to_str(s: *const MyStruct) -> *const c_char {
    let s : Box<MyStruct> = mem::transmute(s);
    let s = CString::new(format!("{:?}", s)).unwrap();
    let p = s.as_ptr();
    mem::forget(s);
    p
}
