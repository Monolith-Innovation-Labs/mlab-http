use std::error::Error;
use serde_json::{json, Value};
use reqwest::blocking::Client;
use create::file_utils::*;
use create::types::*;

pub fn send_request(client: &Client, request: &Request) -> Result<reqwest::blocking::Response, String> {
    let response = match request.method.as_str() {
        "get" => client.get(&request.url),
        "post" => client.post(&request.url),
        "put" => client.put(&request.url),
        "patch" => client.patch(&request.url),
        "delete" => client.delete(&request.url),
        _ => return Err("Unknown method!".to_string())
    };

    response.json(&request.data).send().map_err(|err| err.to_string())
}

pub fn handle_requests(client: &Client, input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let file = read_file(input)?;

    for request in iter_request_file(&file) {
        #[cfg(feature = "log")]
        println!("Send request: {:?}", &request);

        let response = match send_request(client, &request) {
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

        #[cfg(feature = "log")]
        println!("Response: {}", serde_json::to_string_pretty(&response)?);

        let response = "\n".to_string() + &serde_json::to_string(&response)?;

        append_to_file(output, &response)?;
    }

    Ok(())
}
