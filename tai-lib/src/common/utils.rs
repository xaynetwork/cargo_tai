pub fn envs_as_string(envs: &[(String, String)]) -> String {
    envs.iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join(" ")
}
