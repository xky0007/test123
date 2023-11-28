
use std::net::TcpStream;
use std::env;

use std::collections::HashMap;
fn main() {
    let args: Vec<String> = env::args().collect();
    let args_as_str: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();    
    let mut addr:  &str = "1";
    if args_as_str.len() > 1
    {
        addr = args_as_str[1];
    }
    else{
        println!("Please input the ip and port e.g. 127.0.0.1:80");
        return;
    }
    check(addr);
}
fn check(addr: &str)  {
    let stream = TcpStream::connect(addr);
    // stream.write(&[1])?;
    // let mut buf = [0; 128];
    // match stream.read(&mut buf) {
    //     Ok(_) => println!("Port is open!"),
    //     Err(_) => println!("Port is closed!"),
    // }
    match stream {
        Ok(_) => println!("Port is open"),
        Err(_) => println!("Port is closed"),
    }
}
