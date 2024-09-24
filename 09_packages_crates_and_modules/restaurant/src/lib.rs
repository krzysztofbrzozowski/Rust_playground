mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path to call add_to_waitlist
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path to call add_to_waitlist
    front_of_house::hosting::add_to_waitlist();
}