# h10

Simple HTTP/1.0 Server with no external dependencies (WIP)

## Getting Started


### Installation
```
cargo install h10
```

### Installed mode
```
./h10-server --ip-address=127.0.0.1 --port=9000 
```
or 
```
./h10-server --help
```
### Caution

The usage of `--http1.0` argument will make the request from browsers return 400 (Bad Request) code.
```
./h10-server --http1.0 --ip-address=127.0.0.1 --port=9000 
```

### Usage
```
h10 (v0.3.4): Simple HTTP/1.0 Server - https://github.com/ancos2505/h10

Usage: h10-server [OPTION]

Options:
  --help                      Display this message

  --verbose                   Show raw contents from both Request and Response

  --http1.0                   Enable strict HTTP/1.0 strict mode (RFC1945)
                              WARNING: The nowday's browsers doesn't use it anymore

  --ip-address=<IP ADDRESS>   IPv4 or IPv6 to listening

  --port=<PORT NUMBER>        Port to listen [1024-65535] (RFC7605#section-4)

```
### Dev mode

#### Terminal 1
```
cargo run
```
#### Terminal 2
```
curl -v localhost:8080
```

or open in your browser: http://localhost:8080/

![Opened in browser](/docs/imgs/opened_in_browser.png)
## Roadmap

- Serve static files sandboxed in a specific folder
- Upload files
- Implement the complete spec [RFC1945](https://www.rfc-editor.org/rfc/rfc1945.html)
