# parrot-network

Experimental public Distributed Ledger Technology (DLT)

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
Experimental public Distributed Ledger Technology (DLT)

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
cargo run -- --config-file=./misc/parrot-node.toml
```
#### Terminal 2
```
curl -v localhost:9000
```

or open webui in your browser: http://localhost:9000/

![Opened in browser](/docs/imgs/opened_in_browser.png)

## Roadmap
- [ ] Deploy DevNet
- [ ] Implement cli for both node and wallet management
- [ ] Implement Webui for node management

