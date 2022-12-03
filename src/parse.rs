use std::fs;

pub fn list_from_string(data: &str) -> Vec<i32> {
    return data.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
}

pub fn list_from_file(path: &str) -> Vec<i32> {
    let contents = fs::read_to_string(path);
    return list_from_string(contents.expect("Failed to read").as_str());
}