#[cfg(target_os = "hermit")]
extern crate hermit_sys;
extern crate hermit_abi;

/// läuft mit uhyve, crasht mit qemu weil kein secure random number device
/// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:11:67
fn main() {
        println!("Hello World!");
        unsafe {
            for i in 0..9 {
                let random :u32 = hermit_abi::rand();
                println!("{}: {}", i,  random);
                // let new_random :u32 = hermit_abi::secure_rand32().unwrap();
                println!("{}: {}", i,  new_random);

            }
        }
}
