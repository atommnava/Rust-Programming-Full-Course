fn main()
{
    
    enum IpAddrKind {
        V4(String),
        V6(String),
    }
     
    struct ipAddr {
        kind: IpAddrKind,
        address: String
    }
    
    let _four: ipAddrKind = ipAddrKind::V4;
    let _six: ipAddrKind = ipAddrKind::V6;
    
    fn route(ip_kind: ipAddrKind){}
    route(ipAddrKind::V4);
    route(ipAddrKind::V6);
    let home: IpAddr = ipAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback: IpAddr = ipAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    println!("");
}