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
Parrot Network Node (v0.0.5): Experimental public blockchain built on top of HTTP/1.0 (RFC-1945) protocol - https://github.com/ancos2505/parrot-network

Usage: parrot-node [OPTION]

Options:
  --help                      Display this message

  --verbose                   Show raw contents from both Request and Response

  --webui-ip=<IP ADDRESS>     IPv4 or IPv6 to listening

  --webui-port=<PORT NUMBER>  Port to listen [1024-65535] (RFC7605#section-4)

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
- [ ] Implement Webui for node management
- [ ] Implement the complete spec [RFC1945](https://www.rfc-editor.org/rfc/rfc1945.html)
