#[cfg(target_os = "hermit")]
extern crate hermit_sys;
use std::arch::asm;

fn main() {
    println!("Main at {:p}", main as *const ());

    // test rw to data segment 
    unsafe {
        println!("===.data===");
        static mut MOSTLY :&str = "harmless";
        println!("MOSTLY: {} at {:p}", MOSTLY, MOSTLY as *const str);
        println!("Trying to modify MOSTLY...");
        MOSTLY = "AAAAAAAA";
        println!("MOSTLY: {} at {:p}", MOSTLY, MOSTLY as *const str);
        static DATAVAR :i64 = 0xccc3d3ff;
        let func_ptr = &DATAVAR;
        // transmut pointer to value in .data segment to a function pointer
        let func: extern "C" fn() = std::mem::transmute(func_ptr);
        // move address of function to call to rbx
        asm!("mov rbx, {0}", in(reg) call_target as *const ());
        // This works:
        // asm!("call rbx");
        // call the function pointer on the heap
        // This does not work!
        // func();
    }

    // test rw to heap segment
    println!("===heap===");
    let mut so_long = String::from("and thanks for all the fish!");
    println!("so_long: {} at {:p}", so_long, &so_long as *const String); 
    println!("Trying to modify so_long...");
    so_long = String::from("AAAAAAAA");
    println!("so_long: {} at {:p}", so_long, &so_long as *const String); 
    // test x heap segment
    // call rbx;
    let heapvar = Box::<i64>::new(0xccc3d3ff);
    unsafe {
        let func_ptr = heapvar;
        // transmut pointer to value on the heap to a function pointer
        let func: extern "C" fn() = std::mem::transmute(func_ptr);
        // move address of function to call to rbx
        asm!("mov rbx, {0}", in(reg) call_target as *const ());
        // This works:
        // asm!("call rbx");
        // call the function pointer on the heap
        // This does not work!
        // func();
    }

    // test rw to text segment, x is obvious
    println!("===.text===");
    unsafe {
        println!("build_hyperspace_route(): {:#x} at {:p}", *(build_hyperspace_route as *mut i64), build_hyperspace_route as *const ());
    }
    println!("Trying to modify build_hyperspace_route()...");
    unsafe {
        // call rbx; ret; int3; 
       *(build_hyperspace_route as *mut i64) = 0xccc3d3ff;
        println!("build_hyperspace_route(): {:#x} at {:p}", *(build_hyperspace_route as *mut i64), build_hyperspace_route as *const ());
        println!("Call build_hyperspace_route()");
        // move address of target function to rbx
        asm!("mov rbx, {0}", in(reg) call_target as *const ());
        // This works in uhyve:
        // asm!("call rbx");
        // call modified function
        // the modified code is executed, target function is called, but it breaks somwhere later
        // in uhyve; however in qemu it works
        build_hyperspace_route();
        println!("Returned from build_hyperspace_route()");
    }

    // test rw to stack segment
    println!("===stack===");
    let mut meaning_of_life :i32 = 42;
    println!("meaning_of_life: {} at {:p}", meaning_of_life, &meaning_of_life as *const i32);
    println!("Trying to modify meaning_of_life...");
    meaning_of_life = 0x41;
    println!("meaning_of_life: {} at {:p}", meaning_of_life, &meaning_of_life as *const i32);
    // test x stack segment
    // call rbx; ret; int3;
    let stackvar :i64 = 0xccc3d3ff;
    unsafe {
        let func_ptr = &stackvar;
        // transmut i64 pointer to function pointer
        let func: extern "C" fn() = std::mem::transmute(func_ptr);
        // move address of function to call to rbx
        asm!("mov rbx, {0}", in(reg) call_target as *const ());
        // This works:
        // asm!("call rbx");
        // call the function pointer on the stack
        // This does not work!
        // func();
    }
}

#[inline(never)]
fn build_hyperspace_route() {
    return
}


#[inline(never)]
fn call_target() {
    println!("Called target function!");
}
