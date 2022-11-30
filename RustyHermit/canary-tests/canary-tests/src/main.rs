/**
 *  Simple function call to analyze the resulting disassembly for stack canaries
 **/

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
    println!("Main function address: {:p}", main as *const ());
    let has_passierschein = check_passierschein(38);    
    println!("Access? {}", has_passierschein);
}

#[inline(never)]
fn check_passierschein(number: i32) -> bool {
    println!{"Sie benötigen Passierschein A{}!", number}
    return false;
}
