#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::env;

    use std::fs::read_to_string;
    use tai_resource::include_file;
    use test_project::test_resources;

    #[test]
    fn test_data_host_and_device() {
        test_resources();
        let data = include_file!("data/test.txt");
        let data_nested = include_file!("data/data-nested/test.txt");

        let text = read_to_string(data).unwrap();
        assert_eq!("84", &text);
        let text_nested = read_to_string(data_nested).unwrap();
        assert_eq!("8484", &text_nested);
    }
}
