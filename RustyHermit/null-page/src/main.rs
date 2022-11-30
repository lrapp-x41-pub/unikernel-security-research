#[cfg(target_os = "hermit")]
use hermit_sys as _;

fn main() {
    unsafe{
        // Uncomment for testing, might produce page faults
        
        println!("Read NULL page:");
        println!("0x{:x}", *(0x0 as *const usize));
        
        // println!("Write NULL page:");
        // *(0x0 as *mut u8) = 0x42;

        // println!("Execute NULL page");
        // let func: extern "C" fn() = std::mem::transmute(0x0 as usize);
        // func();
    }
}
