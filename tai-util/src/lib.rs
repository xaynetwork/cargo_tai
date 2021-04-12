use std::{env, path::PathBuf};

pub const DATA_DIR_NAME: &'static str = "test-data";

pub fn resources_file_path(test_data_id: &str) -> PathBuf {
    try_resources_file_path(test_data_id)
        .expect(&format!("Couldn't find test data {}", test_data_id))
}

pub fn try_resources_file_path(test_data_id: &str) -> Option<PathBuf> {
    let current_exe = env::current_exe().expect("current exe path not accessible");

    if cfg!(any(target_os = "ios", target_os = "android")) {
        current_exe
            .parent()
            .map(|p| p.join(DATA_DIR_NAME))
            .map(|p| p.join(test_data_id))
    } else {
        None
    }
}

#[cfg(target_os = "ios")]
pub mod ios {
    use ns_path_utilities_sys::{
        INSArray,
        INSString,
        NSSearchPathDirectory,
        NSSearchPathDirectory_NSDocumentDirectory,
        NSSearchPathDirectory_NSLibraryDirectory,
        NSSearchPathDomainMask,
        NSSearchPathDomainMask_NSUserDomainMask,
        NSSearchPathForDirectoriesInDomains,
        NSString,
        NSString_NSStringExtensionMethods,
    };
    use std::{os::raw::c_char, path::PathBuf};

    pub fn user_library() -> PathBuf {
        get_path_for_documents(
            NSSearchPathDirectory_NSLibraryDirectory,
            NSSearchPathDomainMask_NSUserDomainMask,
        )
    }

    pub fn user_documents() -> PathBuf {
        get_path_for_documents(
            NSSearchPathDirectory_NSDocumentDirectory,
            NSSearchPathDomainMask_NSUserDomainMask,
        )
    }

    fn get_path_for_documents(
        directory: NSSearchPathDirectory,
        domain_mask: NSSearchPathDomainMask,
    ) -> PathBuf {
        let search = unsafe { NSSearchPathForDirectoriesInDomains(directory, domain_mask, true) };
        let path_ptr = unsafe { INSArray::<NSString>::objectAtIndex_(&search, 0) };
        let ns_string = NSString(path_ptr);
        let bytes_ptr = unsafe {
            let bytes: *const c_char = ns_string.UTF8String();
            bytes as *const u8
        };
        let path = unsafe {
            let len = ns_string.length();
            let bytes = std::slice::from_raw_parts(bytes_ptr, len as usize);
            std::str::from_utf8(bytes).unwrap()
        };
        PathBuf::from(path)
    }
}
