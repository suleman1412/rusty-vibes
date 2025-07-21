// ipsniffer -h
// ipsniffer -j threads addr
// ipsniffer ipaddr
use std::{
    env,
    net::{IpAddr, TcpStream},
    str::FromStr,
    sync::mpsc::channel,
    thread,
};
const MAX_PORTS: u16 = 65535;
#[derive(Debug)]
enum FlagParseError {
    UnknownFlag,
}
struct Arguments {
    flags: Option<Flags>,
    threads: u16,
    ip: IpAddr,
}
#[derive(Debug)]
enum Flags {
    H,
    J,
}
impl FromStr for Flags {
    type Err = FlagParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-h" | "--help" => Ok(Flags::H),
            "-j" => Ok(Flags::J),
            _ => Err(FlagParseError::UnknownFlag),
        }
    }
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.is_empty() {
            return Err("Need arguments. Use -h for help");
        }
        // ipsniffer ipaddr
        if let Ok(ipaddr) = IpAddr::from_str(&args[0]) {
            if args.len() != 1 {
                return Err("Too Many Arguments for direct IP Scan");
            }
            return Ok(Arguments {
                flags: None,
                threads: 4,
                ip: ipaddr,
            });
        }

        // -h -j
        // ipsniffer -h
        // ipsniffer -j threads addr
        if let Ok(flag) = Flags::from_str(&args[0]) {
            match flag {
                Flags::H => {
                    if args.len() != 1 {
                        return Err("Too Many Arguments for -h");
                    }
                    println!("Usage:");
                    println!(
                        "  ipsniffer <IP_ADDRESS>       - Scan all ports on the given IP address with 4 threads."
                    );
                    println!(
                        "  ipsniffer -j <THREADS> <IP_ADDRESS> - Scan all ports on the given IP address with specified number of threads."
                    );
                    println!("  ipsniffer -h                 - Display this help message.");
                    return Err("Help displayed.");
                }
                Flags::J => {
                    if args.len() != 3 {
                        return Err("'-j' flag requires <THREADS> and <IP_ADDRESS> arguments.");
                    }
                    let num_threads = &args[1]
                        .parse::<u16>()
                        .map_err(|_| "Invalid number of threads")?;
                    let ipaddr = &args[2]
                        .parse::<IpAddr>()
                        .map_err(|_| "Invalid IP address")?;
                    return Ok(Arguments {
                        flags: Some(flag),
                        threads: *num_threads,
                        ip: *ipaddr,
                    });
                }
            }
        }
        // If none of the above matched
        Err("Invalid arguments. Use '-h' for help.")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Pass slice of arguments, excluding the program name itself (args[0])
    match Arguments::new(&args[1..]) {
        Ok(arguments) => scanner(arguments),
        Err(e) => {
            // Only print the error if it's not the "Help displayed." special case
            if e != "Help displayed." {
                eprintln!("Error: {}", e);
            }
            // Exit with a non-zero code to indicate an error for CLI tools
            std::process::exit(1);
        }
    }
}

// Now that you have parsed the arguments, you need to finally start sniffing the ports
// Since I will be spawning multiple threads, in each thread I will be runnign a for loop (or could write some logic on distributing the laod among the specified number of threads) till the MAX_PORT, where ever the TCPStream connects, I will be using mpsc channels to sennd the port number

fn scanner(parsed_arguments: Arguments) {
    let (tx, rx) = channel::<u16>();
    for _ in 1..parsed_arguments.threads {
        let tx1 = tx.clone();
        thread::spawn(move || {
            for i in 0..MAX_PORTS {
                if let Ok(_) = TcpStream::connect((parsed_arguments.ip, i)) {
                    tx1.send(i).unwrap();
                }
            }
        });
    }
    drop(tx);
    let mut listening_ports = vec![];
    for port in rx {
        listening_ports.push(port);
    }
    listening_ports.sort();
    println!("Ports are: {:?}", listening_ports);
}
