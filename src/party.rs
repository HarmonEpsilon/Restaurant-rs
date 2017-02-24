use dlist;

pub mod party {
    pub struct Party {
        reservation_name: Option<Box<&'static str>>,
        num_diners: u32,
        time_req: u32,
    }

    impl Party {
        //default constructor
        pub fn new() -> Party {
            Party {reservation_name: None, num_diners: 0, time_req: 0}
        }

        //create a Party
        pub fn create(reservation: &'static str, diners: u32, time: u32) -> Party {
            Party {reservation_name: Some(reservation), num_diners: diners, time_req: time}
        }

        //get reservation name
        pub fn get_reservation(&self) -> &'static str {
            &self.reservation_name
        }

        //get number of diners
        pub fn get_diners(&self) -> u32 {
            self.num_diners
        }

        //get time required
        pub fn get_time(&self) -> u32 {
            self.time_req
        }
    }
}