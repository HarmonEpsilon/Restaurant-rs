pub mod party {
    use dlist;
    use std::string::String;

    pub struct Party {
        reservation_name: Option<String>,
        num_diners: u32,
        time_req: u32,
    }

    //implementing clone for party
    impl Clone for Party {
        fn clone(&self) -> Self {
            Party {
                reservation_name: self.reservation_name.clone(),
                num_diners: self.num_diners,
                time_req: self.time_req
            }
        }
    }

    impl Party {
        //default constructor
        pub fn new() -> Party {
            Party {reservation_name: None, num_diners: 0, time_req: 0}
        }

        //create a Party
        pub fn create(reservation: String, diners: u32, time: u32) -> Party {
            Party {reservation_name: Some(reservation), num_diners: diners, time_req: time}
        }

        //get reservation name
        pub fn get_reservation(&mut self) -> String {
            self.reservation_name.clone().unwrap()
        }

        //get number of diners
        pub fn get_diners(&mut self) -> u32 {
            self.num_diners
        }

        //get time required
        pub fn get_time(&mut self) -> u32 {
            self.time_req
        }
    }
}