/*
ENUMS : a varsatile tool used to represent a type that can take on one 
of severel possible variants.
 */
fn main() {
    enum IpAddrKind {
        v4,
        v6
    };

    let _four =  IpAddrKind :: v4 ;
    let _six  =  IpAddrKind :: v6 ;

    //using enums with functuons 

    fn route (ipkind : IpAddrKind) {}

    route(IpAddrKind::v4);
    route(IpAddrKind::v6);

    //using struct with enum

    struct IpAddr{
    kind : IpAddrKind ,
    address : String,
};

// using instances with enum & struct :

let home :  IpAddr = IpAddr{
    kind : IpAddrKind::v4,
    address : String :: from ("127.0.0.1"),
};

let loopback : IpAddr = IpAddr {
    kind : IpAddrKind::v6,
    address : String :: from ("::1"),
};

/* 
 we can define the enum type asw 

 enum IpAddr {
    v4 (String),
    v6 (String),
 }

 let home =  IpAddr  :: v4 (String ::  from ("127.0.0.1"));

*/

// enhances enums :

enum IpAddress {
    v4 (u8,u8,u8,u8)
}

/*
There’s another advantage to using an enum rather than a struct: Each variant can have 
different types and amounts of associated data. Version four IP addresses will always have 
four numeric components that will have values between 0 and 255. If we wanted to store
V4 addresses as four u8 values but still express V6 addresses as one String value, we 
wouldn’t be able to with a struct. Enums handle this case with ease:
*/
    enum IpAddresss {
        V44(u8, u8, u8, u8),
        V666(String),
    }

    let home = IpAddresss::V44(127, 0, 0, 1);

    let loopback = IpAddresss::V66(String::from("::1"));


}

