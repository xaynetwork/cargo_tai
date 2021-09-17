use std::panic::catch_unwind;
use test_project::tests::test_data_host_and_device;

#[no_mangle]
pub extern "C" fn run_cargo_tai_runner() -> i32 {
    match catch_unwind(test_data_host_and_device) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}
