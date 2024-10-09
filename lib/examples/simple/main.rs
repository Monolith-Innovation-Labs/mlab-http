use std::thread::sleep;
use std::time::Duration;
use reqwest::blocking::Client;
use mlab_http::handle_requests;

//To test example write `{ "id": 0, "method": "get", "url": "https://swapi.dev/api/films/1" }` into the examples/simple/gamedata/configs/mlab_http_output.txt file

fn main() {
    let input  = "./examples/simple/gamedata/configs/mlab_http_output.txt";
    let output  = "./examples/simple/gamedata/configs/mlab_http_input.txt";

    let client = Client::new();

    println!("Listening...");

    loop {
        sleep(Duration::from_millis(250));

        if let Err(err) = handle_requests(&client, input, output) {
            println!("ERROR: {}", err);
        }
    }
}