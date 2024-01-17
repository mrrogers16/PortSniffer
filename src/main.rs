// Valid Inputs
// ip_sniffer.exe -h
// ip_sniffer.exe -j 100 192.168.1.1
// ip_sniffer.exe 192.168.1.1

use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;


const MAX: u16 = 65535;
struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}
impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("ERROR: Not enough arguments\n");
        } else if args.len() > 4 {
            return Err("ERROR: Too many arguments\n");
        }
        let f: String = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "Usage: -j to slect how many threads you want
                \r\n             -h or -help to show this help message\n"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("ERROR: Too many arguments .contains");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("ERROR: Not a valid IPADDR: must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("ERROR: Failed to parse thread number"),
                };
                return Ok(Arguments {
                    threads,
                    flag,
                    ipaddr,
                });
            } else {
                return Err("ERROR: Invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    
    let mut port: u16 = start_port + 1; 
    let timeout = Duration::from_millis(500);

    loop {
        let socket_addr = (addr, port);
        let connection_result = TcpStream::connect_timeout(&socket_addr.into(), timeout);

        match connection_result {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) < num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );
    let num_threads: u16 = arguments.threads;
    let addr = arguments.ipaddr;
    let(tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }
    println!("");

    out.sort();

    for v in out {
        println!("{} is open", v);
    }
}

