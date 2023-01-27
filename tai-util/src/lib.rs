use std::{env, path::PathBuf};

// cannot use resources https://stackoverflow.com/questions/29271548/code-sign-error-bundle-format-unrecognized-invalid-or-unsuitable
pub const DATA_DIR_NAME: &str = "test-data";
pub const LIB_DIR_NAME: &str = "lib";

pub fn resources_file_path(test_data_id: &str) -> PathBuf {
    try_resources_file_path(test_data_id)
        .unwrap_or_else(|| panic!("couldn't find test data {}", test_data_id))
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
    //! https://docs.microsoft.com/de-de/xamarin/ios/app-fundamentals/file-system#application-directories
    //! https://medium.com/@anandin02/ios-storage-best-practices-294fca83ad9
    use ns_path_utilities_sys::{
        INSArray,
        INSString,
        NSSearchPathDirectory,
        NSSearchPathDirectory_NSApplicationSupportDirectory,
        NSSearchPathDirectory_NSCachesDirectory,
        NSSearchPathDirectory_NSDocumentDirectory,
        NSSearchPathDomainMask,
        NSSearchPathDomainMask_NSUserDomainMask,
        NSSearchPathForDirectoriesInDomains,
        NSString,
        NSString_NSStringExtensionMethods,
    };
    use std::{os::raw::c_char, path::PathBuf};

    pub fn user_documents() -> PathBuf {
        get_path_for_documents(
            NSSearchPathDirectory_NSDocumentDirectory,
            NSSearchPathDomainMask_NSUserDomainMask,
        )
    }

    pub fn user_application_support() -> PathBuf {
        get_path_for_documents(
            NSSearchPathDirectory_NSApplicationSupportDirectory,
            NSSearchPathDomainMask_NSUserDomainMask,
        )
    }

    pub fn user_cache() -> PathBuf {
        get_path_for_documents(
            NSSearchPathDirectory_NSCachesDirectory,
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
