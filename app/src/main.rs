use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    id: u32,
    url: String,
    method: String,
    data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    id: u32,
    url: String,
    method: String,
    data: Option<Value>,
    status: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestError {
    error: String
}

fn read_file(file: &str) -> std::io::Result<String> {
    let data = fs::read_to_string(file)?;
    fs::write(file, "")?;

    Ok(data)
}

fn append_to_file(file: &str, data: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file)?;

    file.write(data.as_bytes())?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Listening...");
    let client = reqwest::blocking::Client::new();

    loop {
        sleep(Duration::from_millis(250));

        let Ok(file) = read_file("./gamedata/configs/crc_output.txt") else {
            println!("Error read");
            continue
        };

        for line in file.split("\n") {
            if line.len() == 0 {
                break;
            }

            let request: Request = serde_json::from_str(line)?;
            println!("Send request: {:?}", request);

            let response = match request.method.as_str() {
                "get" => client.get(&request.url),
                "post" => client.post(&request.url),
                "put" => client.put(&request.url),
                "patch" => client.patch(&request.url),
                "delete" => client.delete(&request.url),
                _ => { continue }
            };

            let response = response.json(&request.data).send();

            let response = match response {
                Ok(res) => (res.status().as_u16(), res.json::<Value>().ok()),
                Err(err) => (500, Some(json!({ "error": err.to_string() }))),
            };

            let response = Response {
                id: request.id,
                url: request.url,
                method: request.method,
                status: response.0,
                data: response.1,
            };

            println!("Response: {}", serde_json::to_string_pretty(&response)?);
            let response = "\n".to_string() + &serde_json::to_string(&response)?;

            let Ok(_) = append_to_file("./gamedata/configs/crc_input.txt", &response) else {
                println!("Error write");
                continue
            };
        };
    }
}