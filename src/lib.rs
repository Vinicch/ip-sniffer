use std::{
    io::{self, Write},
    net::{IpAddr, TcpStream},
    str::FromStr,
    sync::mpsc::Sender,
};

const MAX: u16 = 65535;

pub struct Arguments {
    pub flag: String,
    pub ip_addr: IpAddr,
    pub threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();

        if let Ok(ip_addr) = IpAddr::from_str(&f) {
            Ok(Arguments {
                flag: String::from(""),
                ip_addr,
                threads: 4,
            })
        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "Usage: -j to select how many threads you want
                \r\n      -h or -help to show this help message"
                );

                Err("help")
            } else if flag.contains("-h") || flag.contains("-help") {
                Err("too many arguments")
            } else if flag.contains("-j") {
                let ip_addr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };

                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number"),
                };

                Ok(Arguments {
                    threads,
                    flag,
                    ip_addr,
                })
            } else {
                Err("invalid syntax")
            }
        }
    }
}

pub fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        if TcpStream::connect((addr, port)).is_ok() {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}
