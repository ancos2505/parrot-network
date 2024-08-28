# parrot-network

Experimental public blockchain built on top of HTTP/1.0 (RFC-1945) protocol (WIP)

## Getting Started


### Installation
```sh
$ cargo install parrot-network
```

### Installed mode
```sh
$ parrot-node
```
or 
```sh
$ parrot-node --help
```

### Usage
```
Experimental public blockchain built on top of HTTP/1.0 (RFC-1945) protocol

Usage: parrot-node [OPTIONS]

Options:
      --webui-ip=<WEBUI_IP>        [default: 127.0.0.1]
      --webui-port=<WEBUI_PORT>    [default: 9000]
      --server-ip=<SERVER_IP>      [default: 0.0.0.0]
      --server-port=<SERVER_PORT>  [default: 8080]
      --config-file=<CONFIG_FILE>  [default: ./parrot-node.toml]
      --verbose                    
  -h, --help                       Print help
  -V, --version                    Print version

```
### Dev mode

#### Terminal 1
```
cargo run
```
#### Terminal 2
```
curl -v localhost:9000
```

or open webui in your browser: http://localhost:9000/

![Opened in browser](/docs/imgs/opened_in_browser.png)
## Roadmap
- [ ] Implement Webui for node management
- [ ] Implement the complete spec [RFC1945](https://www.rfc-editor.org/rfc/rfc1945.html)
