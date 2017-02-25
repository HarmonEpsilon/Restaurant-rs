pub mod table {
    use party::party::Party;
    use dlist;
    use std::string::String;

    pub struct Table {
        table_id: Option<String>,
        server_name: Option<String>,
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
        pub fn create(id: String, seats: u32, server: String) -> Table {
            Table {table_id: Some(id), num_seats: seats, server_name: Some(server), timer: 0, party: None}
        }

        //set timer for a table
        pub fn set_timer(&mut self, duration: u32) {
            self.timer = duration;
        }

        //seats a party at a table
        pub fn seat_party(&mut self, new_party: Box<Party>) {
            self.set_timer(new_party.get_time());
            self.party = Some(new_party);
        }

        //get number of seats
        pub fn get_seats(&mut self) -> u32 {
            self.num_seats
        }

        //get server name 
        pub fn get_server(&mut self) -> String {
            self.server_name.unwrap()
        }

        //get timer
        pub fn get_timer(&mut self) -> u32 {
            self.party.unwrap().get_time()
        }

        //get the party seated at a table
        pub fn get_party(&mut self) -> Box<Party> {
            self.party.unwrap()
        }

        //clear the table for the next customer
        pub fn clear_table(&mut self) {
            self.party = None;
            self.timer = 0;
        }
    }
}