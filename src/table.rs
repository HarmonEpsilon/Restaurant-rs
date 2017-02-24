use dlist;

pub mod table {
    use party::party::Party;

    pub struct Table {
        table_id: Option<Box<&'static str>>,
        server_name: Option<Box<&'static str>>,
        num_seats: u32,
        timer: u32,
        party: Option<Box<Party>>,
    }

    impl Table {
        //default constructor
        pub fn new() -> Table {
            Table {table_id: None, server_name: None, num_seats: 0, timer: 0, party: None}
        }

        //create a table with items
        pub fn create(id: &'static str, seats: u32, server: &'static str) -> Table {
            Table {table_id: id, num_seats: seats, server_name: server, timer: 0, party: None}
        }

        //seats a party at a table
        pub fn seat_party(&mut self, new_party: Box<Party>) {
            set_timer(new_party.get_time());
            self.party = new_party.as_ref();
        }

        //get number of seats
        pub fn get_seats(&self) -> u32 {
            self.num_seats
        }

        //get server name 
        pub fn get_server(&self) -> &'static str {
            self.server_name
        }

        //get timer
        pub fn get_timer(&self) -> u32 {
            self.timer
        }

        //set timer for a table
        pub fn set_timer(duration: u32) {
            timer = duration;
        }

        //get the party seated at a table
        pub fn get_party(&self) -> Party {
            self.party
        }

        //clear the table for the next customer
        pub fn clear_table(&self) {
            self.party = None;
            self.timer = 0;
        }
    }
}