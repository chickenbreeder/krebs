use std::net::SocketAddr;

pub struct Config {
    pub addr: SocketAddr
}

impl Config {
    pub fn with_port(port: u16) -> Config {
        let addr = format!("127.0.0.1:{}", port).parse().expect("socket addr");

        Config {
            addr: addr
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            addr: "127.0.0.1:8000".parse().expect("socket addr")
        }
    }
}