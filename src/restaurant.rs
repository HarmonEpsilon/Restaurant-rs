pub mod restaurant {
    use dlist::dlist::Dlist;
    use party::party::Party;
    use table::table::Table;
    use std::string::String;
    use std::io;
    use std::io::prelude::*;

    macro_rules! scanline {
        ($x:expr) => {
            io::stdin().lock().read_line(&mut $x).unwrap();
        };
    }

    pub struct Restaurant {
        available: Dlist::new(),
        occupied: Dlist::new(),
        waiting: Dlist::new(),
    }

    impl Restaurant {
        pub fn get_input() {
            //string variables
            let mut server_id = String::new();
            let mut name = String::new();
            let mut reservation = String::new();
            let mut id = String::new();
            let mut buffer_in = String::new();

            //unsigned 32-bit integer variables
            let mut serving: u32 = 0;
            let mut timer: u32 = 0;
            let mut seats: u32 = 0;

            //Begin procedures
            scanline!(buffer_in);

            
        }
    }
}