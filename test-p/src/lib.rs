#[cfg(test)]
mod tests {
    use std::env;

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
