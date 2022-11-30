/** This program allocates variables in different memory areas and prints their addresses.
 * (Compile and) run it muiltiple times and compare the printed addresses to check for
 * randomization of memory layout.
 **/

#[cfg(target_os = "hermit")]
extern crate hermit_sys;

fn main() {
    println!("===static data===");
    static MOSTLY :&str = "harmless";
    println!("MOSTLY: {} at {:p}", MOSTLY, MOSTLY as *const str);
    
    println!("===heap===");
    let so_long = String::from("and thanks for all the fish!");
    println!("so_long: {} at {:p}", so_long, &so_long as *const String); 

    println!("===text===");
    println!("build_hyperspace_route() at {:p}", build_hyperspace_route as *const ());

    println!("===stack===");
    let meaning_of_life :i32 = 42;
    println!("meaning_of_life: {} at {:p}", meaning_of_life, &meaning_of_life as *const i32);
}

fn build_hyperspace_route() {
    return
}
