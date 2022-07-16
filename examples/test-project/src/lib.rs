use std::fs::read_to_string;

pub fn test_resources() {
    use tai_resource::include_file;

    let data = include_file!("data/test.txt");
    let data_nested = include_file!("data/data-nested/test.txt");

    let text = read_to_string(data).unwrap();
    assert_eq!("42", &text);
    let text_nested = read_to_string(data_nested).unwrap();
    assert_eq!("4242", &text_nested);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::env;

    use crate::test_resources;

    #[test]
    fn test_data_host_and_device() {
        test_resources()
    }

    #[test]
    fn test_data_host_and_device_include_dir() {
        use include_dir::{include_dir, Dir};
        static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data");
        PROJECT_DIR.get_file("test.txt").unwrap();
    }

    #[cfg(all(test, target_os = "ios", target_arch = "aarch64"))]
    #[test]
    fn test_aarch64_ios() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }

    #[cfg(all(test, target_os = "ios", target_arch = "x86_64"))]
    #[test]
    fn test_x86_64_ios() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }

    #[cfg(all(test, target_os = "android", target_arch = "x86_64"))]
    #[test]
    fn test_x86_64_android() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }

    #[cfg(all(test, target_os = "android", target_arch = "x86"))]
    #[test]
    fn test_x86_android() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }

    #[cfg(all(test, target_os = "android", target_arch = "aarch64"))]
    #[test]
    fn test_aarch64_android() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }

    #[cfg(all(test, target_os = "android", target_arch = "arm"))]
    #[test]
    fn test_arm_android() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }
}
