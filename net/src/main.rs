use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap()
        .expect("what happened");
    
    println!("{}", home);
}
