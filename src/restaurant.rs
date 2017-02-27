pub mod restaurant {
    extern crate input_stream;

    use dlist::dlist::Dlist;
    use party::party::Party;
    use table::table::Table;
    use std::string::String;
    use std::io;
    use self::input_stream::InputStream;

    pub struct Restaurant {
        available: Dlist<Table>::new(),
        occupied: Dlist<Table>::new(),
        waiting: Dlist<Party>::new(),
    }

    impl Restaurant {
        pub fn get_input() {
            //string variables
            let mut server_id: String;
            let mut name: String;
            let mut reservation: String;
            let mut id: String;
            let mut buffer_in: String;

            //unsigned 32-bit integer variables
            let mut serving: u32 = 0;
            let mut timer: u32 = 0;
            let mut seats: u32 = 0;

            //Begin procedures
            
        }
    }
}