#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
    println!("\n\n-----------------------------\n\n");

    unsafe {
        println!("hello_func(): {:#x}", *(hello_func as *mut i64));
    }
    println!("Call hello_func()");
    hello_func();
    
    unsafe {
        // ret; int3; int3; int3;
       *(hello_func as *mut i64) = 0xccccccc3;
        println!("\nhello_func(): {:#x}", *(hello_func as *mut i64));
    }
    println!("Call hello_func()");
    hello_func();

    println!("\n\n-----------------------------\n\n");
}

#[inline(never)]
fn hello_func() {
    println!("Hello, world!");
}
