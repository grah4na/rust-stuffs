mod scanner {   // which represnts module here
    pub mod network {   // pub making it scope public
        pub fn scan_port() {
            println!("Scanning port...");
            super::helper();   // short form of path specifying
        }
    }

    fn helper() {
        println!("Helper function");
    }
}

use crate::scanner::network;

fn main() {
    network::scan_port();
}