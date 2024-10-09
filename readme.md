# HTTP Library for Stalker Anomaly
Stalker library that adds sending HTTP requests via Lua.

## How Does It Work?
The library is divided into two parts. The `gamedata` folder contains a Lua library designed to send requests to the EXE application. The second part, written in Rust, acts as a mediator: it collects requests sent from Lua, sends them out, waits for responses, and then passes them back to Lua.

Communication between Lua and Rust is based on the files `mlab_http_input.txt` and `mlab_http_output.txt`. When you call the function `mlab_http.fetch("GET", "example.com", {}, callback)`, it will write a request to send an HTTP request into the `mlab_http_output.txt` file. The Rust application will then read this file and execute the request, and the response will be saved to the `mlab_http_input.txt` file. Lua checks this file every 250 ms; if it is not empty, it checks if the callback has not been canceled, and then executes the callback function.

## Functions
### `mlab_http.fetch(method, url, data, callback)` 
Sends an HTTP request. 
Returns: 
`Request { id: number, method: HttpMethod, url: string, data: table | nil, callback: fun(data: Response), cancel: fun() }`. 
The callback will be executed automatically. It is possible to cancel the request via `Request.cancel`.

## Examples
Full examples can be found in the `lib/examples` folder.

### How to Build the EXE Application?
We need to check `mlab_http_input.txt` in a loop every 250 ms; the library handles everything for you.

Minimal code:

```rust
use std::thread::sleep;
use std::time::Duration;
use reqwest::blocking::Client;
use mlab_http::handle_requests;

fn main() {
    let input = "./gamedata/configs/mlab_http_output.txt";
    let output = "./gamedata/configs/mlab_http_input.txt";

    let client = Client::new();

    loop {
        sleep(Duration::from_millis(250));

        if let Err(err) = handle_requests(&client, input, output) {
            println!("ERROR: {}", err);
        }
    }
}
```

### How to Use It in Stalker?
When you press the `o` key, an HTTP request will be sent to the Star Wars database. When the request is completed, you will see information about the first film in the log. The library currently only supports working with data in JSON format, so the response body is automatically converted to a Lua table. To print this data, we need to convert it back to a string.

```lua
function on_game_start()
    RegisterScriptCallback("on_key_press", on_key_press)
end

function on_key_press(key)
    if key == DIK_keys.DIK_O then 
        mlab_http.fetch("GET", "https://swapi.dev/api/films/1", nil, function (res)
            news_manager.send_tip(db.actor, ("Callback -> url: %s, status: %s, data: %s"):format(res.url, res.status, mlab_json.stringify(res.data)), nil, nil, 30000)
        end)
    end
end
```

## Planed features
- [] make request async