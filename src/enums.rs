// Enums
// versatile tool used to represent a type that 
// can be one of several variants

fn main() {
    
    // Enum
    enum IpAddrKind {
        V4,
        V6,
    }

    let _four: IpAddrKind = IpAddrKind::V4;
    let _six: IpAddrKind = IpAddrKind::V6;

    // Error? - Expected one of `V4`, `V6`, found `IpAddrKind::V4`
    // fn route(_ip_kind: IpAddrKind) {
    //     println!("Route: {:?}", _ip_kind);
    // }

    // route(_ip_kind: IpAddrKind::V4);
    // route(_ip_kind: IpAddrKind::V6);

    struct IpAddr{
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


    // Cleaner way to represent the IpAddrKind
    enum IPAddress {
        V4(String),
        V6(String),
    }
    // Using Enums
    let home: IPAddress = IPAddress::V4(String::from("127.0.0.1"));
    let loopback: IPAddress = IPAddress::V6(String::from("::1"));

    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    // Enhanced enums
    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

}
