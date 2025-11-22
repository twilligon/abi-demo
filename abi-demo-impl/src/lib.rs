#![deny(improper_ctypes)]

use std::{mem, ptr};

use abi_demo_lib::{abi_to_vec, vec_to_abi, Summer, VecAbi, __SummerVtable};

struct Summer1;

impl Summer for Summer1 {
    fn sum(&mut self, v: Vec<i32>) -> i32 {
        v.into_iter().sum()
    }
}

#[no_mangle]
pub extern "C-unwind" fn get_summer1() -> *mut () {
    Box::into_raw(Box::new(Summer1)) as *mut ()
}

#[no_mangle]
pub extern "C-unwind" fn get_summer2() -> *mut () {
    Box::into_raw(Box::new(Summer2 { grand_total: 0 })) as *mut ()
}

struct Summer2 {
    grand_total: i32,
}

impl Summer for Summer2 {
    fn sum(&mut self, v: Vec<i32>) -> i32 {
        let sum: i32 = v.into_iter().sum();
        self.grand_total += sum;
        sum
    }
}

impl Drop for Summer2 {
    #[inline(never)]
    fn drop(&mut self) {
        if self.grand_total > 0 {
            dbg!(self.grand_total);
        }
    }
}

#[allow(non_snake_case)]
extern "C-unwind" fn __Summer_sum<T: Summer>(ptr: *mut (), v: VecAbi<i32>) -> i32 {
    unsafe { &mut *(ptr as *mut T) }.sum(abi_to_vec(v))
}

#[allow(non_snake_case)]
unsafe extern "C-unwind" fn __Drop_drop<T>(ptr: *mut ()) {
    ptr::drop_in_place(ptr as *mut T);
}

#[no_mangle]
pub extern "C-unwind" fn get_rust_vtable1() -> *const __SummerVtable {
    let summer: &dyn Summer = &Summer1;
    unsafe { mem::transmute::<&dyn Summer, (*const (), *const __SummerVtable)>(summer) }.1
}

#[no_mangle]
pub extern "C-unwind" fn get_our_vtable1() -> *const __SummerVtable {
    &const {
        __SummerVtable {
            drop: if mem::needs_drop::<Summer1>() {
                Some(__Drop_drop::<Summer1>)
            } else {
                None
            },
            size: mem::size_of::<Summer1>(),
            align: mem::align_of::<Summer1>(),
            __Summer_sum: __Summer_sum::<Summer1>,
        }
    } as *const __SummerVtable
}

#[no_mangle]
pub extern "C-unwind" fn get_rust_vtable2() -> *const __SummerVtable {
    let mut summer_val = Summer2 { grand_total: 0 };
    let summer: &mut dyn Summer = &mut summer_val;
    unsafe { mem::transmute::<&mut dyn Summer, (*mut (), *const __SummerVtable)>(summer) }.1
}

#[no_mangle]
pub extern "C-unwind" fn get_our_vtable2() -> *const __SummerVtable {
    &const {
        __SummerVtable {
            drop: if mem::needs_drop::<Summer2>() {
                Some(__Drop_drop::<Summer2>)
            } else {
                None
            },
            size: mem::size_of::<Summer2>(),
            align: mem::align_of::<Summer2>(),
            __Summer_sum: __Summer_sum::<Summer2>,
        }
    } as *const __SummerVtable
}

fn doubled(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().map(|x| x * 2).collect()
}

#[no_mangle]
pub extern "C-unwind" fn __doubled(v: VecAbi<i32>) -> VecAbi<i32> {
    vec_to_abi(doubled(abi_to_vec(v)))
}

#[no_mangle]
pub extern "C-unwind" fn get_doubled_addr() -> *const () {
    doubled as *const ()
}

#[no_mangle]
pub extern "C-unwind" fn get_doubled_abi_addr() -> *const () {
    __doubled as *const ()
}
