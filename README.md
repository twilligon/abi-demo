# abi-demo

We have zero-cost rust<->rust stable ABI at home:

    $ cargo run --release
        Finished `release` profile [optimized] target(s) in 0.04s
         Running `target/release/abi-demo`
    [src/main.rs:23:9] get_rust_vtable1() = 0x0000ffff792e6de8
    [src/main.rs:23:9] &*get_rust_vtable1() = __SummerVtable {
        drop: None,
        size: 0,
        align: 1,
        __Summer_sum: 0x0000ffff792a26f4,
    }
    [src/main.rs:24:9] get_rust_vtable2() = 0x0000ffff792e6e08
    [src/main.rs:24:9] &*get_rust_vtable2() = __SummerVtable {
        drop: Some(
            0x0000ffff792a261c,
        ),
        size: 4,
        align: 4,
        __Summer_sum: 0x0000ffff792a2624,
    }
    [src/main.rs:25:9] get_our_vtable1() = 0x0000ffff792e6da8
    [src/main.rs:25:9] &*get_our_vtable1() = __SummerVtable {
        drop: None,
        size: 0,
        align: 1,
        __Summer_sum: 0x0000ffff792a26f4,
    }
    [src/main.rs:26:9] get_our_vtable2() = 0x0000ffff792e6dc8
    [src/main.rs:26:9] &*get_our_vtable2() = __SummerVtable {
        drop: Some(
            0x0000ffff792a261c,
        ),
        size: 4,
        align: 4,
        __Summer_sum: 0x0000ffff792a2624,
    }

    [src/main.rs:29:9] get_doubled_addr() = 0x0000ffff792a27b0
    [src/main.rs:29:9] get_doubled_abi_addr() = 0x0000ffff792a27b0

    [src/main.rs:32:9] doubled(vec![1, 2, 3, 4, 5]) = [
        2,
        4,
        6,
        8,
        10,
    ]

    [src/main.rs:38:9] (vtable1.__Summer_sum)(summer1, vec_to_abi(vec![1, 2, 3, 4, 5])) = 15

    [src/main.rs:58:9] (vtable2.__Summer_sum)(summer2, vec_to_abi(vec![6, 7, 8, 9, 10])) = 40
    [abi-demo-impl/src/lib.rs:41:13] self.grand_total = 40

This crate is basically a very elaborate way to do nothing. Why bother? Well, so your failure mode is "you juggle a few registers at the ABI boundary" rather than "your program instantly blows up".

Rust does not have a stable ABI (except when using `#[repr(C)]`). If you yolo arbitrary types across an ABI boundary (e.g. to or from a dynamically-loaded plugin), they can have *different representations on each side*. rustc reserves the right to do this for reasons as arbitrary as changed flags or profile or version or build or phase of the moon or [running out of gas](https://github.com/rust-lang/rust/pull/40377). So your plugin might silently reinterpret a `Vec<i32>`'s `ptr`, `len`, `cap` as `len`, `ptr`, `cap`. Suddenly your `Vec` has trillions of entries stored at 0x0000000000000005. Needless to say, this is Undefined Behavior.

But if at the ABI boundary we convert every `Vec` to and from a `#[repr(C)] struct VecAbi` that *just so happens* to exactly match Vec's representation *on my machine*, there are two possibilities:

- If we're lucky (i.e. your rustc lays out `Vec` same as mine), conversions are optimized into nothing but a memcpy, then eliminated entirely by standard LLVM optimizations. ABI-safe wrappers for functions taking or returning Vec are optimized into simple calls to the underlying function that takes a `Vec`, then inlined copies of said functions, then optimized away entirely by identical code folding when the linker sees the wrapper's machine code is byte-for-byte identical to the underlying function.
- If we're unlucky, we have to move a couple integers around so `Vec`'s arbitrary representation matches `VecAbi`'s defined representation. We pay one function call, a few register swaps, and a few bytes in code size.

So like the yolo approach, we bet on "nice coincidences" which work the vast majority of the time. But unlike the yolo approach, the only cost to being wrong is (very slight!) performance rather than correctness.

Stay tuned for macros that make these gymnastics easy.
