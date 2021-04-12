use std::{env, fs::File, io::Read, path::PathBuf};

pub fn resources_file_path(test_data_id: &str) -> PathBuf {
    try_resources_file_path(test_data_id)
        .expect(&format!("Couldn't find test data {}", test_data_id))
}

pub fn try_resources_file_path(test_data_id: &str) -> Option<PathBuf> {
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
