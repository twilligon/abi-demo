#![deny(improper_ctypes)]

use core::mem::ManuallyDrop;

#[repr(C)]
pub struct VecAbi<T> {
    cap: usize,
    ptr: *mut T,
    len: usize,
}

impl<T> Drop for VecAbi<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Vec::from_raw_parts(self.ptr, self.len, self.cap));
        }
    }
}

pub fn abi_to_vec<T>(abi: VecAbi<T>) -> Vec<T> {
    let abi = ManuallyDrop::new(abi);
    unsafe { Vec::from_raw_parts(abi.ptr, abi.len, abi.cap) }
}

pub fn vec_to_abi<T>(vec: Vec<T>) -> VecAbi<T> {
    let vec = ManuallyDrop::new(vec);
    VecAbi {
        cap: vec.capacity(),
        ptr: vec.as_ptr() as *mut T,
        len: vec.len(),
    }
}

pub trait Summer {
    fn sum(&mut self, v: Vec<i32>) -> i32;
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct __SummerVtable {
    pub drop: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pub size: usize,
    pub align: usize,
    pub __Summer_sum: unsafe extern "C-unwind" fn(*mut (), VecAbi<i32>) -> i32,
}

extern "C-unwind" {
    fn __doubled(v: VecAbi<i32>) -> VecAbi<i32>;
}

pub fn doubled(v: Vec<i32>) -> Vec<i32> {
    unsafe { abi_to_vec(__doubled(vec_to_abi(v))) }
}
