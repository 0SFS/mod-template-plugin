#![no_std]

use core::panic::PanicInfo;
use core::ptr::addr_of_mut;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut INPUT: [f64; 8] = [0.0; 8];
static mut OUTPUT: [f64; 8] = [0.0; 8];

#[no_mangle]
pub extern "C" fn osfs_mod_abi_version() -> i32 {
    1
}

#[no_mangle]
pub extern "C" fn osfs_mod_init(_sample_rate_hz: f64, _inputs: i32, _outputs: i32) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn osfs_mod_reset() {}

#[no_mangle]
pub extern "C" fn osfs_mod_input_ptr() -> *mut f64 {
    addr_of_mut!(INPUT).cast()
}

#[no_mangle]
pub extern "C" fn osfs_mod_output_ptr() -> *mut f64 {
    addr_of_mut!(OUTPUT).cast()
}

#[no_mangle]
pub extern "C" fn osfs_mod_step(dt_sec: f64) -> i32 {
    unsafe {
        let input = addr_of_mut!(INPUT).cast::<f64>().read();
        addr_of_mut!(OUTPUT).cast::<f64>().write(input + dt_sec);
    }
    0
}
