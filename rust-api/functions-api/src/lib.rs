extern crate functions_core;

use functions_core::hello_from_rust as core_hello;

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    core_hello();
}