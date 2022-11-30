#![feature(asm)]

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

// This works in qemu but gets stuck with uhyve somewhere inside the target function's print call
fn main() {
    println!("Main at {:p}", main as *const ());
    println!("Call unmodified some_func()");
    some_func(); 
    println!("Returned from unmodified some_func()");
    println!("Call modified some_func()");
    unsafe {
        // overwrite original function consitsing of a single ret instruction with: call rbx; ret; int3;
        *(some_func as *mut i64) = 0xccc3d3ff;
        // move address of target function to rbx
        asm!("mov rbx, {0}", in(reg) call_target as *const ());
        // call modified function
        some_func()
    }
    println!("Returned from modified some_func()");
}

#[inline(never)]
fn some_func() {
    return
}


#[inline(never)]
fn call_target() {
    println!("Called target!");
}
