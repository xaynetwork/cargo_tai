#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::env;

    #[test]
    fn test_data_host_and_device() {
        use std::fs::File;
        use tai_util::try_resources_file_path;
        let path = try_resources_file_path("test_txt").unwrap_or_else(|| "./data/test.txt".into());

        File::open(path).unwrap();
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
    fn test_arm_android() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }
}
