#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::env;

    use std::fs::File;
    use tai_resource::include_file;
    use test_project::test_resources;

    #[test]
    fn test_data_host_and_device() {
        test_resources();
        let path = include_file!("data/test.txt");
        let path2 = include_file!("data/data2/test.txt");

        File::open(path).unwrap();
        File::open(path2).unwrap();
    }
}
