#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
    println!("Hello, world!");
    unsafe {
        let page_table_address: *mut u64 = 0xFFFFFF8000000000 as *mut u64;
        println!("page table content: {}", *page_table_address);
        *page_table_address = 0x41;
        println!("page table content: {}", *page_table_address); 
    } 
}
