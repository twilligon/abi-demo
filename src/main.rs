#![deny(improper_ctypes)]

use abi_demo_lib::{__SummerVtable, doubled, vec_to_abi};
use std::{
    alloc::{Layout, dealloc},
    ptr,
};

#[link(name = "abi_demo_impl")]
unsafe extern "C-unwind" {
    unsafe fn get_rust_vtable1() -> *const __SummerVtable;
    unsafe fn get_our_vtable1() -> *const __SummerVtable;
    unsafe fn get_rust_vtable2() -> *const __SummerVtable;
    unsafe fn get_our_vtable2() -> *const __SummerVtable;
    unsafe fn get_doubled_addr() -> *const ();
    unsafe fn get_doubled_abi_addr() -> *const ();
    unsafe fn get_summer1() -> *mut ();
    unsafe fn get_summer2() -> *mut ();
}

fn main() {
    unsafe {
        dbg!(get_rust_vtable1(), &*get_rust_vtable1());
        dbg!(get_rust_vtable2(), &*get_rust_vtable2());
        dbg!(get_our_vtable1(), &*get_our_vtable1());
        dbg!(get_our_vtable2(), &*get_our_vtable2());

        eprintln!();
        dbg!(get_doubled_addr(), get_doubled_abi_addr());

        eprintln!();
        dbg!(doubled(vec![1, 2, 3, 4, 5]));

        let vtable1 = &*get_our_vtable1();
        let summer1 = get_summer1();

        eprintln!();
        dbg!((vtable1.__Summer_sum)(
            summer1,
            vec_to_abi(vec![1, 2, 3, 4, 5])
        ));

        if let Some(vtable_drop) = vtable1.drop {
            vtable_drop(summer1);
        }

        if vtable1.size > 0 {
            dealloc(
                summer1 as *mut u8,
                Layout::from_size_align(vtable1.size, vtable1.align).unwrap(),
            );
        }

        let vtable2 = &*get_our_vtable2();
        let summer2 = get_summer2();

        eprintln!();
        dbg!((vtable2.__Summer_sum)(
            summer2,
            vec_to_abi(vec![6, 7, 8, 9, 10])
        ));

        if let Some(vtable_drop) = vtable2.drop {
            vtable_drop(summer2);
        }

        if vtable2.size > 0 {
            dealloc(
                summer2 as *mut u8,
                Layout::from_size_align(vtable2.size, vtable2.align).unwrap(),
            );
        }

        if !ptr::fn_addr_eq(
            (*get_rust_vtable1()).__Summer_sum,
            (*get_our_vtable1()).__Summer_sum,
        ) || !ptr::fn_addr_eq(
            (*get_rust_vtable2()).__Summer_sum,
            (*get_our_vtable2()).__Summer_sum,
        ) || !ptr::fn_addr_eq(
            (*get_rust_vtable2()).drop.unwrap(),
            (*get_our_vtable2()).drop.unwrap(),
        ) || get_doubled_addr() != get_doubled_abi_addr()
        {
            eprintln!(
                "\nbuild abi-demo-impl in release mode with ICF=all for \
                identical __Summer_sum, drop, doubled"
            );
        }
    }
}
