#[cfg(any(feature = "tai-tests", test))]
pub mod tests {
    #[allow(unused_imports)]
    use std::env;

    #[cfg_attr(test, test)]
    pub fn test_data_host_and_device() {
        use std::fs::File;
        use tai_util::try_resources_file_path;
        let path = try_resources_file_path("test_txt").unwrap_or_else(|| "./data/test.txt".into());

        File::open(path).unwrap();
    }

    #[cfg_attr(test, test)]
    #[cfg(all(target_os = "ios", target_arch = "aarch64"))]
    fn test_aarch64_ios() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }

    #[cfg_attr(test, test)]
    #[cfg(all(target_os = "ios", target_arch = "x86_64"))]
    fn test_x86_64_ios() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }

    #[cfg_attr(test, test)]
    #[cfg(all(target_os = "android", target_arch = "x86_64"))]
    fn test_x86_64_android() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }

    #[cfg_attr(test, test)]
    #[cfg(all(target_os = "android", target_arch = "x86"))]
    fn test_x86_android() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }

    #[cfg_attr(test, test)]
    #[cfg(all(target_os = "android", target_arch = "aarch64"))]
    fn test_aarch64_android() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }

    #[cfg_attr(test, test)]
    #[cfg(all(target_os = "android", target_arch = "arm"))]
    fn test_arm_android() {
        assert_eq!(2 + 2, 4);
        assert_eq!(env::var("TAI").unwrap_or(String::from("1")), "1");
        assert_eq!(env::var("TAI2").unwrap_or(String::from("2")), "2");
    }
}
