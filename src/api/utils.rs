pub fn add_param(base: &mut String, name: &str, value: &Option<String>) {
    if let Some(val) = value {
        base.push_str(&format!("&{}={}", name, val));
    }
}
