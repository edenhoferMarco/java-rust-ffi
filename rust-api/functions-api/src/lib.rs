extern crate functions_core;

use functions_core::get_calculation_handle as core_get_calculation_handle;
use functions_core::hello_from_rust as core_hello;
use functions_core::perform_calculation as core_perform_calculation;
use functions_core::CalculationHandle;
use functions_core::CalculationType as CoreCalculationType;

use std::ffi::c_void;

#[repr(C)]
pub enum CalculationType {
    Add = 0,
    Subtract = 1,
}

/// Create a new calculation handle. The returned pointer is an owned pointer and must be
/// passed to `perform_calculation` (which takes ownership and frees it) or
/// `free_calculation_handle` to avoid leaks.
#[no_mangle]
pub unsafe extern "C" fn get_calculation_handle(
    calc_type: CalculationType,
    inputs_ptr: *const i32,
    inputs_len: usize,
) -> *mut c_void {
    let inputs_slice = unsafe {
        if inputs_ptr.is_null() || inputs_len == 0 {
            &[] as &[i32]
        } else {
            std::slice::from_raw_parts(inputs_ptr, inputs_len)
        }
    };

    let inputs_vec = inputs_slice.to_vec();

    let core_type = match calc_type {
        CalculationType::Add => CoreCalculationType::Add,
        CalculationType::Subtract => CoreCalculationType::Subtract,
    };

    let handle = core_get_calculation_handle(core_type, inputs_vec);
    Box::into_raw(Box::new(handle)) as *mut c_void
}

/// Performs the calculation and frees the handle (ownership is consumed).
#[no_mangle]
pub unsafe extern "C" fn perform_calculation(handle_ptr: *mut c_void) -> i32 {
    if handle_ptr.is_null() {
        return 0;
    }

    // Recreate the boxed handle (this takes ownership and will free it when dropped)
    let boxed: Box<CalculationHandle> =
        unsafe { Box::from_raw(handle_ptr as *mut CalculationHandle) };
    core_perform_calculation(*boxed)
}

/// Free a previously allocated calculation handle without performing the calculation.
#[no_mangle]
pub unsafe extern "C" fn free_calculation_handle(handle_ptr: *mut c_void) {
    if handle_ptr.is_null() {
        return;
    }
    unsafe {
        // Recreate the boxed handle to drop it
        let _ = Box::from_raw(handle_ptr as *mut CalculationHandle);
    }
}

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    core_hello();
}
