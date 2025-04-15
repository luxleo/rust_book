enum IpAddrKind {
    V4(IpV4Addr),
    V6(IpV6Addr)
}

#[derive(Debug)]
struct IpV4Addr {
    address: (u16,u16,u16,u16)
}
impl IpV4Addr{
    fn new(address: (u16,u16,u16,u16)) -> Self {
        IpV4Addr{address}
    }
}
#[derive(Debug)]
struct IpV6Addr {
    address: String
}

pub fn show_enum() {
    let addr1 = IpAddrKind::V4(IpV4Addr::new((127,0,0,1)));
    let addr2 = IpAddrKind::V4(IpV4Addr::new((127,0,0,1)));
}