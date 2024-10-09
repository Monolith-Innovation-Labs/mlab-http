use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use crate::Request;

pub fn read_file(file: &str) -> std::io::Result<String> {
    let data = fs::read_to_string(file)?;
    fs::write(file, "")?;

    Ok(data)
}

pub fn append_to_file(file: &str, data: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file)?;

    file.write(data.as_bytes())?;

    Ok(())
}

pub fn iter_request_file<'a>(file: &'a str) -> impl Iterator<Item=Request> + 'a {
    file.split('\n').filter_map(|line| {
        if line.is_empty() {
            return None
        }

        match serde_json::from_str::<Request>(line) {
            Ok(request) => Some(request),
            Err(_) => None,
        }
    })
}