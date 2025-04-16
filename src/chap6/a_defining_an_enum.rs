enum IpAddrKind {
    V4(IpV4Addr),
    V6(IpV6Addr)
}
impl IpAddrKind {
    fn info(&self){
        match self {
            IpAddrKind::V4(ip) => println!("IPv4 address: {}", ip.to_string()),
            IpAddrKind::V6(ip) => println!("IPv6 address: {}", ip.address)
        }
    }
}
#[derive(Debug)]
struct IpV4Addr {
    address: (u16,u16,u16,u16)
}
impl IpV4Addr{
    fn new(address: (u16,u16,u16,u16)) -> Self {
        IpV4Addr{address}
    }
    fn to_string(&self)->String{
        let mut ret = String::new();
        ret.push_str(self.address.0.to_string().as_str());
        ret
    }
}
#[derive(Debug)]
struct IpV6Addr {
    address: String
}
impl IpV6Addr {
    fn new(address: String) -> IpV6Addr {
        IpV6Addr{address}
    }
}

pub fn show_enum() {
    let addr1 = IpAddrKind::V4(IpV4Addr::new((127,0,0,1)));
    let addr2 = IpAddrKind::V4(IpV4Addr::new((127,0,0,1)));
    addr1.info();
    addr2.info();
}

// Option and Null
//So why is having Option<T> any better than having null?
//
// In short, because Option<T> and T (where T can be any type) are different types,
// the compiler won’t let us use an Option<T> value as if it were definitely a valid value.
// For example, this code won’t compile, because it’s trying to add an i8 to an Option<i8>:
fn option_is_better_than_null() {
    let x : u8 = 255;
    let y : Option<u8> = Some(1);
    // let z = x-y;
}