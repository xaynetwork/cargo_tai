use std::{env, fs::File, io::prelude::*, path::PathBuf};

pub fn test_file_path(test_data_id: &str) -> PathBuf {
    try_test_file_path(test_data_id).expect(&format!("Couldn't find test data {}", test_data_id))
}

pub fn try_test_file_path(test_data_id: &str) -> Option<PathBuf> {
    let current_exe = env::current_exe().expect("Current exe path not accessible");

    if cfg!(any(target_os = "ios", target_os = "android")) || env::var("DINGHY").is_ok() {
        current_exe
            .parent()
            .map(|it| it.join("test_data"))
            .map(|it| it.join(test_data_id))
    } else {
        let test_data_path = current_exe
            .parent()
            .and_then(|it| it.parent())
            .map(|it| it.join("dinghy"))
            .map(|it| it.join(current_exe.file_name().unwrap()))
            .map(|it| it.join("test_data"));
        let test_data_path = match test_data_path {
            None => return None,
            Some(test_data_cfg_path) => test_data_cfg_path,
        };

        let test_data_cfg_path = test_data_path.join("test_data.cfg");

        let mut contents = String::new();
        let test_data_cfg =
            File::open(&test_data_cfg_path).and_then(|mut f| f.read_to_string(&mut contents));
        if let Err(_) = test_data_cfg {
            return None;
        }

        contents
            .lines()
            .map(|line| line.split(':'))
            .map(|mut line| (line.next(), line.next()))
            .find(|&(id, _)| id.map(|it| it == test_data_id).unwrap_or(false))
            .and_then(|(_, path)| path)
            .map(PathBuf::from)
    }
}

pub mod ios {
    use ns_path_utilities_sys::{
        INSArray,
        INSString,
        NSSearchPathDirectory_NSDocumentDirectory,
        NSSearchPathDirectory_NSLibraryDirectory,
        NSSearchPathDomainMask_NSUserDomainMask,
        NSSearchPathForDirectoriesInDomains,
        NSString,
        NSString_NSStringExtensionMethods,
    };
    use std::{os::raw::c_char, path::PathBuf};

    pub fn user_library() -> PathBuf {
        let search = unsafe {
            NSSearchPathForDirectoriesInDomains(
                NSSearchPathDirectory_NSLibraryDirectory,
                NSSearchPathDomainMask_NSUserDomainMask,
                true,
            )
        };
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

    pub fn user_documents() -> PathBuf {
        let search = unsafe {
            NSSearchPathForDirectoriesInDomains(
                NSSearchPathDirectory_NSDocumentDirectory,
                NSSearchPathDomainMask_NSUserDomainMask,
                true,
            )
        };
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
